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
use super::Lend;

type DbConn = PooledConnection<SqliteConnectionManager>;

pub fn get_lends(conn: &DbConn, id: Option<String>, search: Option<String>, page: usize) -> Result<models::MainJson<Lend>, Error> {
    let items_per_page = helpers::get_max_items_per_page(&conn);
    let mut sql = "SELECT l.id, l.lend_date, l.planned_return_date, l.actual_return_date, l.id_lab_item, l.id_person, p.firstname, p.lastname FROM lend l
    JOIN person p ON p.matr_nr = l.id_person                    
    ORDER BY lend_date COLLATE NOCASE LIMIT ?2, ?3";

    let mut first_param = String::new();
    let mut items_total: usize = conn.query_row("SELECT Count(*) FROM lend", [], |row| row.get(0)).unwrap();


    if id.is_some() {
        sql = "SELECT l.id, l.lend_date, l.planned_return_date, l.actual_return_date, l.id_lab_item, l.id_person, p.firstname, p.lastname  FROM lend l
        JOIN person p ON p.matr_nr = l.id_person
        WHERE l.id_lab_item = ?1 
        LIMIT ?2, ?3";
        first_param = id.unwrap_or(String::from(""));
        items_total = conn.query_row("SELECT Count(*) FROM lend JOIN person ON person.matr_nr = lend.id_person WHERE lend.id_lab_item = ?1", [&first_param], |row| row.get(0)).unwrap();
    }

    if search.is_some() {
    }

    
    let params = [
        first_param, 
        ((page - 1) * items_per_page).to_string(), 
        items_per_page.to_string()];


    let mut stmt = conn.prepare(sql)?;
    
    let rows = stmt.query_map(params, |row| {
        Ok(Lend {
            id: row.get(0)?,
            lend_date: row.get(1)?,
            planned_return_date: row.get(2)?,
            actual_return_date: row.get(3)?,
            id_lab_item: row.get(4)?,
            id_person: row.get(5)?,
            firstname: row.get(6)?,
            lastname: row.get(7)?,
        })
    })?;
    
    let lends = helpers::collect_rows(rows)?;

    helpers::wrap_pagination(lends, items_total, page, items_per_page)
}


pub fn create_lend(conn: &DbConn, lend: Lend) -> Result<usize, Error> {
    //First check, if person exists:
    let person_exists:i64 = conn.query_row("SELECT EXISTS(SELECT 1 FROM person WHERE matr_nr = ?1)", [&lend.id_person], |row| row.get(0))?;
    if person_exists == 0{
        return Err(Error::InvalidParameterName(String::from("MatNr")))
    }
    //First change borrow status on item
    conn.execute("UPDATE lab_item SET available = 0 WHERE  id= ?1", [&lend.id_lab_item])?;
    let params = [lend.id_lab_item, lend.id_person, lend.lend_date, lend.planned_return_date];
    conn.execute("INSERT INTO lend (id_lab_item, id_person, lend_date, planned_return_date) VALUES (?1, ?2, ?3, ?4)", params)
}

pub fn delete_lend(conn: &DbConn, id: String) -> Result<usize, Error> {
    let params = [id];
    conn.execute("DELETE FROM lend WHERE id = ?1", params)
}

/**
 * just updates the actual return date of a lend, that hasnt been returned
 */
pub fn update_lend(conn: &DbConn, lend: Lend) -> Result<usize, Error> {
    //First get lend_id from lend.id_lab_item, where actual_return_date doesnt exist
        //Will return an Error, if there is no lend with id_lab_item
    let lend_id:i64 = conn.query_row("SELECT id FROM lend WHERE actual_return_date IS NULL AND id_lab_item=?1", [&lend.id_lab_item], |row| row.get(0))?;


    //Now change available status of item back to available
    conn.execute("UPDATE lab_item SET available = 1 WHERE  id= ?1", [&lend.id_lab_item])?;
    conn.execute("UPDATE lend SET actual_return_date = ?1 WHERE id = ?2", [&lend.actual_return_date, &Some(lend_id.to_string())])
}
