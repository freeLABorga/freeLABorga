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

use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Error;


use crate::models;
use crate::helpers;
use super::Gegenstand;

type DbConn = PooledConnection<SqliteConnectionManager>;



pub fn get_gegenstaende(conn: &DbConn, page: usize) -> Result<models::MainJson<Gegenstand>, Error> {
    let items_per_page= 99999999;
    let sql = "SELECT id, name, serial_number, price, id_place, available, buy_date, inventoried FROM lab_item ORDER BY name COLLATE NOCASE LIMIT ?2, ?3";

    let first_param = String::new();


    let params = [
        first_param, 
        ((page - 1) * items_per_page).to_string(), 
        items_per_page.to_string()];


    let mut stmt = conn.prepare(sql)?;

    let rows = stmt.query_map(params,|row| {
        let available: bool;
        if row.get::<_, i64>(5).unwrap() == 0 {
            available = false;
        } else {
            available = true;
        }
        let inventoried: bool;
        if row.get::<_, i64>(7).unwrap() == 0 {
            inventoried = false;
        } else {
            inventoried = true;
        }
        Ok(Gegenstand {
            name: row.get(1)?,
            id: row.get(0).expect("get row(0) failed"),
            categories: get_categories_from_id(conn, row.get(0))?,
            place: get_place_from_id(conn, row.get(4))?,
            serialnumber: row.get(2)?,
            available: Some(available),
            price: cents_to_euros(row.get(3))?,
            buydate: row.get(6)?,
            //inventoried: row.get(5)?,
            inventoried: Some(inventoried),
        })
    }).expect("failed to convert to gegenstand");
    let gegenstaende = helpers::collect_rows(rows).unwrap();

    let items_total: usize = conn.query_row("SELECT COUNT(*) FROM lab_item", [],|row| row.get(0)).expect("Query row failed");

    helpers::wrap_pagination(gegenstaende, items_total, page, items_per_page)

}

fn get_categories_from_id(conn: &DbConn, id: Result<String, rusqlite::Error>) -> Result<Vec<String>, rusqlite::Error> {
    let id = id?;

    //Now get all categories by name, that the item has
    let sql = "SELECT name FROM category JOIN item_cat ON item_cat.id_category = category.id WHERE item_cat.id_item = ?1 ORDER BY category.name";
    let mut stmt = conn.prepare(sql)?;

    let rows = stmt.query_map([id],|row| {
        row.get(0)
    })?;
    helpers::collect_rows(rows)
}

fn get_place_from_id(conn: &DbConn, id: Result<i64, rusqlite::Error>) -> Result<String, rusqlite::Error> {
    let id = id?;

    let sql = "SELECT name FROM storage_place WHERE id=?1";

    let storage_name = conn.query_row(sql, [id],|row| row.get(0)).unwrap();
    Ok(storage_name)
}

fn cents_to_euros(cents: Result<i64, rusqlite::Error>) -> Result<String, rusqlite::Error> {
    let cents = cents?;
    let euros = cents / 100;
    let cents = cents % 100;
    Ok(format!("{:},{:02} €", euros, cents))
}
