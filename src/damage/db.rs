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
use super::Damage;

type DbConn = PooledConnection<SqliteConnectionManager>;

pub fn get_damages(conn: &DbConn, id: Option<String>, search: Option<String>, page: usize) -> Result<models::MainJson<Damage>, Error> {
    let items_per_page = helpers::get_max_items_per_page(&conn);

    let mut sql = "SELECT id, description, repaired, date, lab_item_id FROM damage ORDER BY date COLLATE NOCASE LIMIT ?2, ?3";

    let mut first_param = String::new();

    if id.is_some() {
        sql = "SELECT id, description, repaired, date, lab_item_id FROM damage WHERE id = ?1 LIMIT ?2, ?3";
        first_param = id.unwrap_or(String::from(""));
    }

    if search.is_some() {
        sql = "SELECT id, description, repaired, date, lab_item_id FROM damage WHERE lab_item_id = ?1 LIMIT ?2, ?3";
        first_param = search.unwrap_or(String::from(""));
    }

    println!("{}", &first_param);
    
    let params = [
        first_param, 
        ((page - 1) * items_per_page).to_string(), 
        items_per_page.to_string()];


    let mut stmt = conn.prepare(sql)?;
    
    let rows = stmt.query_map(params, |row| {
        Ok(Damage {
            id: row.get(0)?,
            description: row.get(1)?,
            repaired: row.get::<_, u16>(2)? != 0, // int to bool
            date: row.get(3)?,
            lab_item_id: row.get(4)?,
        })
    })?;
    
    let damages = helpers::collect_rows(rows)?;

    let items_total: usize = conn.query_row("SELECT Count(*) FROM damage", [], |row| row.get(0)).unwrap();

    helpers::wrap_pagination(damages, items_total, page, items_per_page)
}


pub fn create_damage(conn: &DbConn, damage: Damage) -> Result<usize, Error> {
    let params = (damage.description, damage.repaired as u16, damage.date, damage.lab_item_id);
    conn.execute("INSERT INTO damage (description, repaired, date, lab_item_id) VALUES (?1, ?2, ?3, ?4)", params)
}

pub fn delete_damage(conn: &DbConn, id: String) -> Result<usize, Error> {
    let params = [id];
    conn.execute("DELETE FROM damage WHERE id = ?1", params)
}

pub fn update_damage(conn: &DbConn, old_id: String, damage: Damage) -> Result<usize, Error> {
    let params = (damage.description, damage.repaired as u16, damage.date, damage.lab_item_id, old_id);
    conn.execute("UPDATE damage SET description = ?1, repaired = ?2, date = ?3, lab_item_id = ?4 WHERE id = ?5", params)
}
