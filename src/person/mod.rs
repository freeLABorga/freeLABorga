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

mod routes;
mod tests;
mod db;

pub use routes::init_routes;
use serde::{Serialize, Deserialize};
use crate::models;


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    pub matr_nr: String,
    pub lastname: String,
    pub firstname: String,
    pub email: String,
}

impl Person {
    pub fn validate(&self) -> models::OperationResult {
        // Auf leere Felder prüfen
        let mut empty_fields = Vec::new();
        if self.matr_nr.is_empty() {empty_fields.push("matrNr");}
        if self.lastname.is_empty() {empty_fields.push("lastname");}
        if self.firstname.is_empty() {empty_fields.push("firstname");}
        if self.email.is_empty() {empty_fields.push("email");}
        
        if !empty_fields.is_empty() {
            let message = format!("Empty fields: {}", empty_fields.join(", "));
            return models::OperationResult {
                success: false,
                message,
            };
        }

        // E-Mail überprüfen
        let re = regex::Regex::new(r"^\w+([\.-]?\w+)*@\w+([\.-]?\w+)*(\.\w+)+$").unwrap();
        if !re.is_match(&self.email) {
            return models::OperationResult {
                success: false,
                message: String::from("incorrect email"),
            };
        }

        // Im Erfolgsfall:
        models::OperationResult {
            success: true,
            message: String::from("OK"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AnonymizePerson {
    pub person_available: bool,
    pub person: Option<Person>,
    pub days: Option<i64>,
}
