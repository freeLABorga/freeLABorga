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
use super::Place;

type DbConn = PooledConnection<SqliteConnectionManager>;



pub fn get_places(conn: &DbConn, id: Option<String>, search: Option<String>, page: usize) -> Result<models::MainJson<Place>, Error> {
    let items_per_page = helpers::get_max_items_per_page(&conn);

    const SELECT_SQL: &str = "SELECT storage_place.id, storage_place.name, count(lab_item.id) FROM storage_place 
                              LEFT OUTER JOIN lab_item ON lab_item.id_place = storage_place.id 
                              {} 
                              GROUP BY storage_place.id
                              ORDER BY storage_place.id
                              LIMIT ?, ?";
    const COUNT_SQL: &str = "SELECT Count(*) FROM storage_place {}";

    let mut where_clause = "";
    let mut params = Vec::new();

    if id.is_some() {
        where_clause = "WHERE storage_place.id = ?1";
        params.push(id.unwrap_or(String::from("")));

    } else if search.is_some() {
        where_clause = "WHERE 
                        storage_place.id LIKE ?1 OR 
                        storage_place.name LIKE ?1";  
        params.push(format!("%{}%", search.unwrap_or(String::from(""))));
    }

    params.push(((page - 1) * items_per_page).to_string());
    params.push(items_per_page.to_string());
    let select_params_iter = rusqlite::params_from_iter(&params);
    
    let mut stmt = conn.prepare(&SELECT_SQL.replace("{}", where_clause))?;
    
    let rows = stmt.query_map(select_params_iter, |row| {
        Ok(Place {
            id: row.get(0)?,
            name: row.get(1)?,
            number: Some(row.get(2)?),
        })
    })?;
    
    let places = helpers::collect_rows(rows)?;
    
    let count_params_iter = rusqlite::params_from_iter(&params[..params.len()-2]);

    let mut count_stmt = conn.prepare(&COUNT_SQL.replace("{}", where_clause))?;
    let items_total: usize = count_stmt.query_row(count_params_iter, |row| row.get(0))?;

    helpers::wrap_pagination(places, items_total, page, items_per_page)
}


pub fn create_place(conn: &DbConn, place: Place) -> Result<usize, Error> {
    let params = [place.name];
    conn.execute("INSERT INTO storage_place (name) VALUES (?1)", params)
}

pub fn delete_place(conn: &DbConn, id: String) -> Result<usize, Error> {
    let params = [id];
    conn.execute("DELETE FROM storage_place WHERE id = ?1", params)
}

pub fn update_place(conn: &DbConn, old_id: String, place: Place) -> Result<usize, Error> {
    let params = [place.name, old_id];
    conn.execute("UPDATE storage_place SET name = ?1 WHERE id = ?2", params)
}
