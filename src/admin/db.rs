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

use super::Admin;

type DbConn = PooledConnection<SqliteConnectionManager>;



pub fn get_admin_texts(conn: &DbConn) -> Result<Admin, Error> {
    let mut admin_obj = Admin::default();

    // Temporäre Variablen anlegen, um zwischen String (aus DB) und INT/BOOL (für Struct) zu konvertieren
    let mut is_html = ("0".to_string(), "0".to_string());
    let mut items_per_page = String::new();
    let mut days_until_anonymize = String::new();

    let admin_items = [
        ("imprint_text", &mut admin_obj.imprint_text),
        ("imprint_html", &mut is_html.0),
        ("privacy_text", &mut admin_obj.privacy_text),
        ("privacy_html", &mut is_html.1),
        ("items_per_page", &mut items_per_page),
        ("days_until_anonymize", &mut days_until_anonymize),
    ];
    let mut stmt = conn.prepare("SELECT value FROM config WHERE key = ?")?;


    for item in admin_items {
        let val = stmt.query_row([item.0],|row| row.get(0)).unwrap_or(String::new());
        *item.1 = val;
    } 

    if is_html.0 == "1".to_string() {
        admin_obj.imprint_html = true;
    }
    if is_html.1 == "1".to_string() {
        admin_obj.privacy_html = true;
    }

    // Nur abspeichern, wenn aus Datenbank gelesener Wert als INT geparst werden kann.
    if let Ok(number) = items_per_page.parse::<usize>() {
        admin_obj.items_per_page = Some(number);
    }
    if let Ok(number) = days_until_anonymize.parse::<usize>() {
        admin_obj.days_until_anonymize = Some(number);
    }

    Ok(admin_obj)
}




pub fn update_admin_texts(conn: &DbConn, admin_obj: Admin) -> Result<(), Error> {
    let is_html = (
        if admin_obj.imprint_html {"1"} else {"0"},
        if admin_obj.privacy_html {"1"} else {"0"},
    );
    let mut admin_items = vec![
        ("imprint_text", admin_obj.imprint_text),
        ("imprint_html", is_html.0.to_string()),
        ("privacy_text", admin_obj.privacy_text),
        ("privacy_html", is_html.1.to_string()),
        ];

    // Nur wenn Werte in Struct vorhanden, in DB eintragen
    if let Some(number) = admin_obj.items_per_page {
        admin_items.push(("items_per_page", number.to_string()));
    }
    if let Some(number) = admin_obj.days_until_anonymize {
        admin_items.push(("days_until_anonymize", number.to_string()));
    }
    
    let mut stmt = conn.prepare("INSERT OR REPLACE INTO config (key, value) VALUES (?1, ?2)")?;

    for item in admin_items {
        stmt.execute(item)?;
    };
    Ok(())
}


pub fn get_config_texts(conn: &DbConn, key: &str) -> Result<String, Error> {
    let mut stmt = conn.prepare("SELECT value FROM config WHERE key = ?1")?;
    stmt.query_row([key], |row| row.get(0))
}
