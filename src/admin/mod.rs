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


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Admin {
    pub imprint_text: String,
    pub imprint_html: bool,
    pub privacy_text: String,
    pub privacy_html: bool,
    pub items_per_page: Option<usize>,
    pub days_until_anonymize: Option<usize>,
}



impl Default for Admin {
    fn default() -> Admin {
        Admin {
            imprint_text: String::new(),
            imprint_html: false,
            privacy_text: String::new(),
            privacy_html: false,
            items_per_page: Some(20),
            days_until_anonymize: Some(100)
        }
    }
}
