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


use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use actix_web::{web, web::Data, HttpResponse};

use crate::helpers;
use crate::models::Info;
use super::Suggestion;
use super::db;

type DbConnPool = Pool<SqliteConnectionManager>;


#[actix_web::get("/schnellsuche")]
async fn get_search_suggestions(pool: Data<DbConnPool>, query: web::Query<Info>) -> HttpResponse {
    let search_query = query.search.clone().unwrap_or("".to_string());
    println!("{}", search_query);
    let result = helpers::db_query(pool, move |conn| { 
        Ok(Suggestion {
            things: db::get_things(conn, &search_query)?,
            persons: db::get_persons(conn, &search_query)?,
        })
        
    }).await;

    helpers::create_return_table_object(result)
}

#[actix_web::get("/kategorie_lagerplatz")]
async fn get_cat_places(pool: Data<DbConnPool>) -> HttpResponse {
    let result = helpers::db_query(pool, move |conn| { 
        db::get_cat_places(conn) 
    }).await;

    helpers::create_return_table_object(result)
}


pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_search_suggestions);
    cfg.service(get_cat_places);
}
