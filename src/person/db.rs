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

use chrono::Duration;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Error;
use chrono::{prelude::Local, Datelike};

use crate::models;
use crate::helpers;
use super::{Person, AnonymizePerson};

type DbConn = PooledConnection<SqliteConnectionManager>;



pub fn get_persons(conn: &DbConn, id: Option<String>, search: Option<String>, page: usize) -> Result<models::MainJson<Person>, Error> {
    let items_per_page = helpers::get_max_items_per_page(&conn);

    const SELECT_SQL: &str = "SELECT matr_nr, lastname, firstname, email FROM person {} ORDER BY lastname COLLATE NOCASE LIMIT ?, ?";
    const COUNT_SQL: &str = "SELECT Count(*) FROM person {}";

    let mut where_clause = "";
    let mut params = Vec::new();

    if id.is_some() {
        where_clause = "WHERE matr_nr = ?1";
        params.push(id.unwrap_or(String::from("")));
    }

    if search.is_some() {
        where_clause = "WHERE 
        matr_nr LIKE ?1 OR 
        lastname LIKE ?1 OR  
        firstname LIKE ?1 OR  
        email LIKE ?1";
        params.push(format!("%{}%", search.unwrap_or(String::from(""))));
    }

    params.push(((page - 1) * items_per_page).to_string());
    params.push(items_per_page.to_string());

    let mut stmt = conn.prepare(&SELECT_SQL.replace("{}", where_clause))?;
    
    let rows = stmt.query_map(rusqlite::params_from_iter(&params), |row| {
        Ok(Person {
            matr_nr: row.get(0)?,
            lastname: row.get(1)?,
            firstname: row.get(2)?,
            email: row.get(3)?,
        })
    })?;
    
    let persons = helpers::collect_rows(rows)?;

    let mut count_stmt = conn.prepare(&COUNT_SQL.replace("{}", where_clause))?;
    let items_total: usize = count_stmt.query_row(rusqlite::params_from_iter(&params[..params.len()-2]), |row| row.get(0))?;

    helpers::wrap_pagination(persons, items_total, page, items_per_page)
}


pub fn create_person(conn: &DbConn, person: Person) -> Result<(), Error> {
    let params = [person.matr_nr, person.lastname, person.firstname, person.email, get_current_date(None)];
    conn.execute("INSERT INTO person (matr_nr, lastname, firstname, email, creation_date) VALUES (?1, ?2, ?3, ?4, ?5)", params)?;
    Ok(())
}


pub fn delete_person(conn: &DbConn, id: String) -> Result<(), Error> {
    let params = [id];
    conn.execute("DELETE FROM person WHERE matr_nr = ?1", params)?;
    Ok(())
}


pub fn update_person(conn: &mut DbConn, old_id: String, person: Person) -> Result<(), Error> {
    let params_update_person = [&person.matr_nr, &person.lastname, &person.firstname, &person.email, &get_current_date(None), &old_id];
    let params_update_lend = [&person.matr_nr, &old_id];
    let tx = conn.transaction()?;
    tx.execute("UPDATE person SET matr_nr = ?1, lastname = ?2, firstname = ?3, email = ?4, creation_date = ?5 WHERE matr_nr = ?6", params_update_person)?;
    tx.execute("UPDATE lend SET id_person = ?1 WHERE id_person = ?2", params_update_lend)?;
    tx.commit()
}


pub fn get_person_to_anonymize(conn: &DbConn) -> Result<AnonymizePerson, Error> {
    const DAYS_UNTIL_ANONYMIZE_SQL: &str = "SELECT CAST(value as INTEGER) FROM config WHERE key = 'days_until_anonymize'";
    const SELECT_SQL: &str = "
    -- Personen, die vor dem XXXX-XX-XX das letzte mal etwas zurückgegeben haben
    SELECT person.matr_nr, person.lastname, person.firstname, person.email
    FROM person
    JOIN lend ON person.matr_nr = lend.id_person
    WHERE person.creation_date IS NOT NULL
    GROUP BY person.matr_nr
    HAVING max(lend.actual_return_date) < ?1
    
    EXCEPT
    
    -- Außer, es sind noch Gegenstände ausgeliehen
    SELECT person.matr_nr, person.lastname, person.firstname, person.email
    FROM person
    JOIN lend ON person.matr_nr = lend.id_person
    WHERE lend.actual_return_date IS NULL
    GROUP BY person.matr_nr
    
    UNION
    
    -- Und Personen, die vor dem XXXX-XX-XX angelegt wurden, aber nie etwas ausgeliehen haben
    SELECT person.matr_nr, person.lastname, person.firstname, person.email
    FROM person
    LEFT JOIN lend ON person.matr_nr = lend.id_person
    WHERE lend.id IS NULL AND person.creation_date IS NOT NULL AND person.creation_date < ?1";

    let days_until_anonymize:i64 = conn.query_row(DAYS_UNTIL_ANONYMIZE_SQL, (), |row| row.get(0)).unwrap_or(100);
    let date_before_anonymize = get_current_date(Some(- days_until_anonymize));
    let mut stmt = conn.prepare(&SELECT_SQL)?;
    let result = stmt.query_row(rusqlite::params_from_iter([&date_before_anonymize]), |row| {
        Ok(Person {
            matr_nr: row.get(0)?,
            lastname: row.get(1)?,
            firstname: row.get(2)?,
            email: row.get(3)?,
        })
    });
    match result {
        Ok(person) => Ok(AnonymizePerson {
            person_available: true, 
            person: Some(person),
            days: Some(days_until_anonymize),
        }),
        Err(Error::QueryReturnedNoRows) => Ok(AnonymizePerson {
            person_available: false, 
            person: None,
            days: None,
        }),
        Err(e) => Err(e),
    }
    
}


/// Schließt Person (übergebene Matrikel-Nr) von der Anonymisierung aus
pub fn exclude_from_anonymisation(conn: &DbConn, id: &str) -> Result<(), Error> {
    conn.execute("UPDATE person SET creation_date = NULL WHERE matr_nr = ?1", [id])?;
    Ok(())
}


/// Gibt Aktuelles Datum + offset_days zurück
/// Ausgabeformat: YYY-MM-DD
fn get_current_date(offset_days: Option<i64>) -> String {
    let local = Local::now() + Duration::days(offset_days.unwrap_or(0));
    format!("{:04}-{:02}-{:02}", local.year(), local.month(), local.day())
}
