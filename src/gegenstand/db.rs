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
use super::{Gegenstand, GegenstandUebersicht};

type DbConn = PooledConnection<SqliteConnectionManager>;



pub fn get_gegenstaende(conn: &DbConn, id: Option<String>, search: Option<String>,item_name: Option<String>, cat: Option<String>, place: Option<String>,  page: usize) -> Result<models::MainJson<Gegenstand>, Error> {
    let items_per_page = helpers::get_max_items_per_page(&conn);

    let mut sql = "SELECT id, name, serial_number, price, id_place, available, buy_date, inventoried FROM lab_item {} ORDER BY name COLLATE NOCASE LIMIT ?, ?";

    let mut params = Vec::new();
    let mut where_clause = "";

    if id.is_some() {
        where_clause = "WHERE id = ?1";
        params.push(id.unwrap_or(String::from("")));
    }

    let mut items_total: usize = conn.query_row("SELECT COUNT(*) FROM lab_item", [],|row| row.get(0)).expect("Query row failed");
    if search.is_some() {
        where_clause = "WHERE
        id LIKE ?1 OR
        name LIKE ?1 OR
        serial_number LIKE ?1 OR
        id_place LIKE ?1 OR
        available LIKE ?1 OR
        buy_date LIKE ?1";
        
        items_total = conn.query_row("SELECT COUNT(*) FROM lab_item WHERE
        id LIKE ?1 OR
        name LIKE ?1 OR
        serial_number LIKE ?1 OR
        id_place LIKE ?1 OR
        available LIKE ?1 OR
        buy_date LIKE ?1", [&search.clone().unwrap()],|row| row.get(0)).expect("Query row failed");

        params.push(format!("%{}%", search.unwrap_or(String::from(""))));
    }

    
    if item_name.is_some() {
        where_clause = "WHERE
        name = ?1";
        params.push(format!("{}", item_name.unwrap_or(String::from(""))));
        items_total = conn.query_row("SELECT COUNT(*) FROM lab_item WHERE name = ?1", [&params[0]],|row| row.get(0)).expect("Query row failed");
    }

    if cat.is_some() {
        sql = "SELECT lab_item.id, lab_item.name, serial_number, price, id_place,
        available, buy_date, inventoried FROM lab_item 
        JOIN item_cat ON lab_item.id = item_cat.id_item 
        JOIN category ON item_cat.id_category = category.id {} ORDER BY lab_item.name COLLATE NOCASE LIMIT ?, ? ";
        where_clause= "WHERE category.name = ?1";
        params.push(format!("{}", cat.unwrap_or(String::from(""))));

        items_total = conn.query_row("SELECT COUNT(*) FROM lab_item JOIN item_cat ON lab_item.id = item_cat.id_item 
        JOIN category ON item_cat.id_category = category.id WHERE category.name = ?1", [&params[0]],|row| row.get(0)).expect("Query row failed");
    }
    if place.is_some() {
        sql = "SELECT lab_item.id, lab_item.name, serial_number, price, id_place,
        available, buy_date, inventoried FROM lab_item 
        JOIN storage_place ON storage_place.id = lab_item.id_place {} ORDER BY lab_item.name COLLATE NOCASE LIMIT ?, ? ";
        where_clause= "WHERE storage_place.name = ?1";
        params.push(format!("{}", place.unwrap_or(String::from(""))));

        items_total = conn.query_row("SELECT COUNT(*) FROM lab_item JOIN storage_place ON storage_place.id = lab_item.id_place WHERE storage_place.name = ?1", [&params[0]],|row| row.get(0)).expect("Query row failed");
    }
    params.push(((page - 1) * items_per_page).to_string());
    params.push(items_per_page.to_string());

    let mut stmt = conn.prepare(&sql.replace("{}", where_clause))?;

    let rows = stmt.query_map(rusqlite::params_from_iter(&params),|row| {
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
            price: row.get::<usize,f64>(3)?/ 100.0,
            buydate: row.get(6)?,
            //inventoried: row.get(5)?,
            inventoried: Some(inventoried),
        })
    })?;
    let gegenstaende = helpers::collect_rows(rows).unwrap();

    

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


pub fn create_gegenstand(conn: &DbConn, gegenstand: Gegenstand) -> Result<usize, Error> {
    //First check if gegenstand.place is already in database. If not, add it
    let place_exists:i64 = conn.query_row("SELECT EXISTS(SELECT 1 FROM storage_place WHERE name = ?1)", [&gegenstand.place], |row| row.get(0))?;
    if place_exists == 0{
        conn.execute("INSERT INTO storage_place (name) VALUES (?1)", [&gegenstand.place])?;
    }
    //add each category if not there and add to cat_item + maintain item_cat table
    for kategorie in gegenstand.categories{
        let kategorie = kategorie.trim();
        if kategorie.is_empty() {
            continue;
        }
        //First check, if category is already there
        let exists:i64 = conn.query_row("SELECT EXISTS(SELECT 1 FROM category WHERE name = ?1)", [&kategorie], |row| row.get(0))?;
        if exists == 0{
            conn.execute("INSERT INTO category (name) VALUES (?1)", [&kategorie]).expect("error inserting into category");
        }
        //Now get the id of the inserted (or old) category
        let cat_id:i64 = conn.query_row("SELECT id FROM category WHERE name = ?1", [&kategorie], |row| row.get(0)).expect("query row cat_id faile");
        //now add row in item_cat
        conn.execute("INSERT INTO item_cat (id_item, id_category) VALUES (?1, ?2)", [gegenstand.id.clone().to_string(),cat_id.to_string()])?;

    }
    //Then add lab item itself
    //--> First get the id for the storage name
    let storage_id:i64 = conn.query_row("SELECT id FROM storage_place WHERE name=?1",[gegenstand.place], |row| row.get(0)).expect("query row storage_id failed");
    //Transform booleans into ints
    let mut available_as_int:i64 = 0;
    match gegenstand.available {
        Some(true) => { available_as_int = 1; },
        Some(false) | None => { /* do nothing */ },
    }

    let mut inventoried_as_int:i64 = 0;
    match gegenstand.inventoried {
        Some(true) => { inventoried_as_int = 1;},
        Some(false) | None => { /* do nothing */},
    }
    let params = (gegenstand.id, gegenstand.name,gegenstand.serialnumber,(&gegenstand.price * 100.0) as i64,storage_id.to_string(),
    available_as_int.to_string(), gegenstand.buydate, inventoried_as_int.to_string());
    conn.execute("INSERT INTO lab_item (id, name, serial_number, price, id_place, available, buy_date, inventoried) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)", params)
}

pub fn delete_gegenstand(conn: &DbConn, id: String) -> Result<usize, Error> {
    //First delete item_cat rows where itemid = id
    conn.execute("DELETE FROM item_cat WHERE id_item = ?1", [&id])?;

    //Then also delete the led history of the item:
    conn.execute("DELETE FROM lend WHERE id_lab_item = ?1", [&id])?;

    //Then also the damage history:
    conn.execute("DELETE FROM damage WHERE lab_item_id = ?1", [&id])?;
    
    //Then delete item itslef
    conn.execute("DELETE FROM lab_item WHERE id = ?1", [&id])
}

pub fn update_gegenstand(conn: &DbConn, old_id: String, gegenstand_in: Gegenstand) -> Result<usize, Error> {
    //First get available and inventoried for old gegenstand
    let available_int:i64 = conn.query_row("SELECT available FROM lab_item WHERE id=?1",[old_id.clone()], |row| row.get(0)).expect("query row avialable failed");
    let inventoried_int:i64 = conn.query_row("SELECT inventoried FROM lab_item WHERE id=?1",[old_id.clone()], |row| row.get(0)).expect("query row avialable failed");
    //turn into bool:
    let mut available = false;
    let mut inventoried = false;
    if available_int == 1 {available = true;}
    if inventoried_int == 1 {inventoried = true;}
    let mut gegenstand = gegenstand_in;
    gegenstand.available = Some(available);
    gegenstand.inventoried = Some(inventoried);

    //Now check if gegenstand.place is already in database. If not, add it
    let place_exists:i64 = conn.query_row("SELECT EXISTS(SELECT 1 FROM storage_place WHERE name = ?1)", [&gegenstand.place], |row| row.get(0))?;
    if place_exists == 0{
        conn.execute("INSERT INTO storage_place (name) VALUES (?1)", [&gegenstand.place])?;
    }
    let storage_id:i64 = conn.query_row("SELECT id FROM storage_place WHERE name=?1",[&gegenstand.place], |row| row.get(0)).expect("query row storage_id failed");

    //hat sich die Gegenstand-id verändert?
    //First delete old cat_item rows
    conn.execute("DELETE FROM item_cat WHERE id_item = ?1", [&old_id])?;

    //Hat sich die id geändert? --> Tabellen mit lab_id anpassen
    if !old_id.eq(&gegenstand.id) {
        conn.execute("UPDATE lend SET id_lab_item = ?1 WHERE id_lab_item = ?2", [&gegenstand.id, &old_id])?;
        conn.execute("UPDATE damage SET lab_item_id = ?1 WHERE lab_item_id = ?2", [&gegenstand.id, &old_id])?;
    }
    for kategorie in gegenstand.categories{
        let kategorie = kategorie.trim();
        if kategorie.is_empty() {
            continue;
        }
        //First check, if category is already there
        let exists:i64 = conn.query_row("SELECT EXISTS(SELECT 1 FROM category WHERE name = ?1)", [&kategorie], |row| row.get(0))?;
        if exists == 0{
            conn.execute("INSERT INTO category (name) VALUES (?1)", [&kategorie]).expect("error inserting into category");
        }
        //Now get the id of the inserted (or old) category
        let cat_id:i64 = conn.query_row("SELECT id FROM category WHERE name = ?1", [&kategorie], |row| row.get(0)).expect("query row cat_id faile");
        //now add row in item_cat
        conn.execute("INSERT INTO item_cat (id_item, id_category) VALUES (?1, ?2)", [gegenstand.id.clone().to_string(),cat_id.to_string()])?;

    }

    

    let params = (&gegenstand.id, gegenstand.name, gegenstand.serialnumber, (&gegenstand.price * 100.0) as i64, gegenstand.buydate, &storage_id, &old_id);
    conn.execute("UPDATE lab_item SET id = ?1, name = ?2, serial_number = ?3, price = ?4, buy_date = ?5, id_place = ?6 WHERE  id= ?7", params)
}




pub fn get_things_overview(conn: &DbConn, page: usize) -> Result<models::MainJson<GegenstandUebersicht>, Error> {
    let items_per_page = helpers::get_max_items_per_page(&conn);

    const SELECT_SQL: &str = "SELECT id, name, Count(*), Sum(available) FROM lab_item WHERE inventoried = 1 GROUP BY name ORDER BY name COLLATE NOCASE LIMIT ?, ?";
    const COUNT_SQL: &str = "SELECT Count(DISTINCT name) FROM lab_item WHERE inventoried = 1";

    let params = (((page - 1) * items_per_page).to_string(), items_per_page.to_string());

    let mut stmt = conn.prepare(&SELECT_SQL)?;
    
    let rows = stmt.query_map(params, |row| {
        Ok(GegenstandUebersicht {
            name: row.get(1)?,
            total: row.get(2)?,
            available: row.get(3)?,
            categories: get_categories_from_id(conn, row.get(0))?,
        })
    })?;
    
    let things_overview = helpers::collect_rows(rows)?;

    let mut count_stmt = conn.prepare(&COUNT_SQL)?;
    let items_total: usize = count_stmt.query_row([], |row| row.get(0))?;

    helpers::wrap_pagination(things_overview, items_total, page, items_per_page)
}

pub fn not_inventory_gegenstand(conn: &DbConn, id: String) -> Result<usize, Error> {
    conn.execute("UPDATE lab_item SET inventoried = 0 WHERE id = ?1", [&id])
}

pub fn inventory_gegenstand(conn: &DbConn, id: String) -> Result<usize, Error> {
    conn.execute("UPDATE lab_item SET inventoried = 1 WHERE id = ?1", [&id])
}
