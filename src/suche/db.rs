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

use crate::helpers;
use super::{SearchItem, CategoriesPlaces};

type DbConn = PooledConnection<SqliteConnectionManager>;

pub fn get_things(conn: &DbConn, search_query: &String) -> Result<Vec<SearchItem>, Error> {
    let sql = "SELECT id, name FROM lab_item WHERE
                    id LIKE ?1 OR
                    name LIKE ?1
                    LIMIT 5";

    let params = [format!("%{}%", search_query)];

    let mut stmt = conn.prepare(sql)?;

    let rows = stmt.query_map(params, |row| {
        Ok(SearchItem {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;

    helpers::collect_rows(rows)
}

pub fn get_persons(conn: &DbConn, search_query: &String) -> Result<Vec<SearchItem>, Error> {
    let sql = "SELECT matr_nr, lastname || ', ' || firstname FROM person WHERE 
                    matr_nr LIKE ?1 OR 
                    lastname LIKE ?1 OR  
                    firstname LIKE ?1 OR  
                    email LIKE ?1
                    LIMIT 5";

    let params = [format!("%{}%", search_query)];

    let mut stmt = conn.prepare(sql)?;

    let rows = stmt.query_map(params, |row| {
        Ok(SearchItem {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;

    helpers::collect_rows(rows)
}


pub fn get_cat_places(conn: &DbConn) -> Result<CategoriesPlaces, Error> {
    const SELECT_CAT_SQL: &str = "SELECT name FROM category"; 
    const SELECT_PLACE_SQL: &str = "SELECT name FROM storage_place";

    let mut stmt = conn.prepare(SELECT_CAT_SQL)?;

    let rows = stmt.query_map([],|row| {
        Ok(row.get(0)?)
    })?;
    let categories = helpers::collect_rows(rows).unwrap();

    stmt = conn.prepare(SELECT_PLACE_SQL)?;

    let rows = stmt.query_map([],|row| {
        Ok(row.get(0)?)
    })?;
    let places = helpers::collect_rows(rows).unwrap();

    Ok(CategoriesPlaces {
        categories,
        place: places,
    })
}
