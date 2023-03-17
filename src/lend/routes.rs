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
use actix_web::{web, web::Data, web::Path, HttpResponse};

use crate::helpers;
use crate::models::Info;
use super::Lend;
use super::db;

type DbConnPool = Pool<SqliteConnectionManager>;

#[actix_web::get("/lend")]
async fn get_lends(pool: Data<DbConnPool>, query: web::Query<Info>) -> HttpResponse {
    let page_number = query.page.unwrap_or(1);
    let result = helpers::db_query(pool, move |conn| { 
        db::get_lends(conn, query.id.clone(), query.search.clone(), page_number) 
    }).await;

    helpers::create_return_table_object(result)
}


#[actix_web::post("/lend/{id}")]
async fn create_lend(pool: Data<DbConnPool>, lend_json: web::Json<Lend>, path: Path<String>) -> HttpResponse {
    let mut lend = lend_json.into_inner();
    lend.id_lab_item = Some(path.into_inner());




    let result = helpers::db_query(pool, move |conn| { 
        db::create_lend(conn, lend) 
    }).await;
    helpers::create_return_status_object(result) 
}

#[actix_web::put("/lend/{id}")]
async fn update_lend(pool: Data<DbConnPool>, lend_json: web::Json<Lend>, path: Path<String>) -> HttpResponse {
    let  mut lend = lend_json.into_inner();
    lend.id_lab_item = Some(path.into_inner());


    let result = helpers::db_query(pool, move |conn| { 
        db::update_lend(conn, lend) 
    }).await;
    helpers::create_return_status_object(result)
       
}


#[actix_web::delete("/lend/{id}")]
async fn delete_lend(pool: Data<DbConnPool>, path: Path<String>) -> HttpResponse {
    let lend_id = path.into_inner();

    let result = helpers::db_query(pool, move |conn| { 
        db::delete_lend(conn, lend_id) 
    }).await;
    helpers::create_return_status_object(result)
} 

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_lends);
    cfg.service(create_lend);
    cfg.service(update_lend);
    cfg.service(delete_lend);
}
