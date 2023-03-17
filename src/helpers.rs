/**
 * This file is part of freeLABorga.
 * Copyright (C) 2022-2023  Nico Hoffmann, Jan Ludwig, Philipp Pfeiffer 
 *
 * freeLABorga is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License Version 3
 * as published by the Free Software Foundation on June 29, 2007.
 *
 * freeLABorga is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with freeLABorga.  If not, see <http://www.gnu.org/licenses/>.
 */


use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Result, MappedRows, Row, Error};
use actix_web::{web, web::Data, HttpResponse, http::StatusCode};
use serde::Serialize;

use crate::models;

type DbConnPool = Pool<SqliteConnectionManager>;
type DbConn = PooledConnection<SqliteConnectionManager>;

const ERROR_MESSAGE_ID_INCONCLUSIVE: &str = "ID bereits vorhanden";
const ERROR_MESSAGE_MATNR_INCONCLUSIVE: &str = "Es gibt keine Person mit dieser MatNr";


// --- WEBSERVER-FUNKTIONEN ---

/// Schnittstelle zwischen Webserver und Datenbankanbindung
/// 
/// Datenbankoperationen können im Gegensatz zum Webserver nicht asynchron ausgeführt werden.
/// Diese Funtkion 
pub async fn db_query<T: std::marker::Send + 'static>(data_pool: Data<DbConnPool>, function: impl FnOnce(&mut DbConn) -> Result<T, rusqlite::Error> + Send + 'static) 
-> Result<T, Box<dyn std::error::Error + Send + Sync>> {
    web::block( move || {
        let mut conn: DbConn = match data_pool.get() {
            Ok(conn) => conn,
            Err(e) => return Err(e.into()),
        };

        match function(&mut conn) {
            Ok(r) => Ok(r),
            Err(e) => return Err(e.into()),
        }

    }).await?
}  


/// Gibt einen HttpRespone mit einem OperationResult-Objekt zurück:
/// Wenn Argument result = Ok(_)      -> OperationResult {success: true, message: "OK",}
/// Wenn Argument result = Err(error) -> OperationResult {success: false, message: error,}
pub fn create_return_status_object<T>(result: Result<T, Box<dyn std::error::Error + Send + Sync>>) -> HttpResponse {
    match result {
        Ok(_) => HttpResponse::Ok().json(
            models::OperationResult {
                success: true,
                message: "OK".to_string(),
            }),
        Err(e) => {
            let mut error_message = format!("{:?}", &e);

            if error_message.contains("SqliteFailure(Error { code: ConstraintViolation, extended_code: 1555 }") {
                error_message = ERROR_MESSAGE_ID_INCONCLUSIVE.to_string();
            }
            if error_message.contains("InvalidParameterName(\"MatNr\")") {
                error_message = ERROR_MESSAGE_MATNR_INCONCLUSIVE.to_string();
            }

            HttpResponse::Ok().json(
            models::OperationResult {
                success: false,
                message: error_message,
            })
        },
    }

}


/// Erstellt HttpResponse mit JSON-Objekt, falls Argument Ok()
/// Wenn Argument result = Ok(json)   -> json
/// Wenn Argument result = Err(error) -> error mit Error Code 500
pub fn create_return_table_object<T: Serialize>(result: Result<T, Box<dyn std::error::Error + Send + Sync>>) -> HttpResponse {
    match result {
        Ok(r) => HttpResponse::Ok().json(r),
        Err(e) => HttpResponse::Ok().status(StatusCode::INTERNAL_SERVER_ERROR).body(format!("{}", e))
    }
}





// --- DATENBANK-FUNKTIONEN ---

/// Erstellt SQL-Tabellen, wenn sie nicht bereits vorhanden sind
/// (Wird v.a. beim Starten benötigt)
pub fn init_tables(conn: DbConn) -> Result<(), Error> {
    conn.execute_batch(models::CREATE_TABLES_SQL)
}


/// Konvertiert ein MappedRows-Struct (von Rusqlite) in einen Vektor
pub fn collect_rows<T, F>(rows: MappedRows<F>) -> Result<Vec<T>, Error> 
where F: FnMut(&Row<'_>) -> Result<T, Error> {
    let mut items = Vec::new();
    for item in rows {
        items.push(item?);
    }
    Ok(items)
}


/// Erstellt ein MainJson-Struct für die Pagination und packt den übergebenen Vektor mit Daten hinein
pub fn wrap_pagination<T>(data: Vec<T>, items_total: usize, page_number: usize, items_per_page: usize) -> Result<models::MainJson<T>, Error> {
    let items_on_page = data.len();
    let pages_total = ((items_total as isize - 1) / items_per_page as isize + 1) as usize;
    
    let wrapper = models::MainJson {
        page_number,
        pages_total,
        items_on_page,
        items_total,
        data,
    };
    Ok(wrapper)
}


/// Liest die maximale Anzahl der Elemente pro Tabellenseite aus der Datenbank
pub fn get_max_items_per_page(conn: &DbConn) -> usize {
    const ITEMS_PER_PAGE_SQL: &str = "SELECT CAST(value as INTEGER) FROM config WHERE key = 'items_per_page'";
    conn.query_row(ITEMS_PER_PAGE_SQL, (), |row| row.get(0)).unwrap_or(20)
}
