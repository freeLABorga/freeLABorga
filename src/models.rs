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

use serde::{Serialize, Deserialize};


/// Wird bei Operationen vom Webserver zurückgegeben, um Erfolg der Operation mitzuteilen
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct OperationResult {
    pub success: bool,
    pub message: String,
}

/// Mögliche GET-Parameter vom Client
#[derive(Deserialize)]
pub struct Info {
    pub page: Option<usize>,
    pub id: Option<String>,
    pub search: Option<String>,
	pub item_name: Option<String>,
	pub cat: Option<String>,
	pub place: Option<String>,
}

/// Wird für die Pagination benötigt
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MainJson<T> {
    pub page_number: usize,
    pub pages_total: usize,
    pub items_on_page: usize,
    pub items_total: usize,
    pub data: Vec<T>,
}


/// SQL Statements erstellen Tabellen, falls sie noch nicht vorhanden sind
/// Sollten beim Starten des Programms ausgeführt werden
pub const CREATE_TABLES_SQL: &str = "
BEGIN TRANSACTION;
CREATE TABLE IF NOT EXISTS 'category' (
	'id'	INTEGER NOT NULL,
	'name'	TEXT,
	PRIMARY KEY('id')
);
CREATE TABLE IF NOT EXISTS 'storage_place' (
	'id'	INTEGER,
	'name'	TEXT,
	PRIMARY KEY('id')
);
CREATE TABLE IF NOT EXISTS 'item_cat' (
	'id_item'	TEXT,
	'id_category'	INTEGER,
	PRIMARY KEY('id_item','id_category')
);
CREATE TABLE IF NOT EXISTS 'lend' (
	'id'	INTEGER NOT NULL,
	'id_lab_item'	INTEGER,
	'id_person'	TEXT,
	'lend_date'	INTEGER,
	'planned_return_date'	TEXT,
	'actual_return_date'	TEXT,
	PRIMARY KEY('id')
);
CREATE TABLE IF NOT EXISTS 'lab_item' (
	'id'	TEXT,
	'name'	TEXT,
	'serial_number'	TEXT,
	'price'	INTEGER,
	'buy_date'	TEXT,
	'inventoried'	INTEGER,
	'available'	INTEGER,
	'id_place'	INTEGER
);
CREATE TABLE IF NOT EXISTS 'config' (
	'key'	TEXT NOT NULL,
	'value'	TEXT NOT NULL,
	PRIMARY KEY('key')
);
CREATE TABLE IF NOT EXISTS 'person' (
	'matr_nr'	TEXT NOT NULL,
	'firstname'	TEXT,
	'lastname'	TEXT,
	'email'	TEXT,
	'creation_date'	TEXT,
	PRIMARY KEY('matr_nr')
);
CREATE TABLE IF NOT EXISTS 'damage' (
	'id'	INTEGER UNIQUE,
	'lab_item_id'	TEXT,
	'date'	TEXT,
	'description'	TEXT,
	'repaired'	TEXT,
	PRIMARY KEY('id' AUTOINCREMENT)
);
CREATE INDEX IF NOT EXISTS 'id' ON 'damage' (
	'id'	ASC
);
COMMIT;
";
