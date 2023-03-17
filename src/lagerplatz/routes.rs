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
use super::Place;
use super::db;

type DbConnPool = Pool<SqliteConnectionManager>;


#[actix_web::get("/lagerplatz")]
async fn get_places(pool: Data<DbConnPool>, query: web::Query<Info>) -> HttpResponse {
    let page_number = query.page.unwrap_or(1);
    let result = helpers::db_query(pool, move |conn| { 
        db::get_places(conn, query.id.clone(), query.search.clone(), page_number) 
    }).await;

    helpers::create_return_table_object(result)
}


#[actix_web::post("/lagerplatz")]
async fn create_place(pool: Data<DbConnPool>, place_json: web::Json<Place>) -> HttpResponse {
    let place = place_json.into_inner();

    let verification_result = place.validate();
    if !verification_result.success {return HttpResponse::Ok().json(verification_result)};

    let result = helpers::db_query(pool, move |conn| { 
        db::create_place(conn, place) 
    }).await;
    helpers::create_return_status_object(result) 
}

#[actix_web::put("/lagerplatz/{id}")]
async fn update_place(pool: Data<DbConnPool>, place_json: web::Json<Place>, path: Path<String>) -> HttpResponse {
    let place = place_json.into_inner();
    let place_id = path.into_inner();

    let verification_result = place.validate();
    if !verification_result.success {return HttpResponse::Ok().json(verification_result)};

    let result = helpers::db_query(pool, move |conn| { 
        db::update_place(conn, place_id, place) 
    }).await;
    helpers::create_return_status_object(result)
       
}


#[actix_web::delete("/lagerplatz/{matr_nr}")]
async fn delete_place(pool: Data<DbConnPool>, path: Path<String>) -> HttpResponse {
    let place_id = path.into_inner();

    let result = helpers::db_query(pool, move |conn| { 
        db::delete_place(conn, place_id) 
    }).await;
    helpers::create_return_status_object(result)
}   


pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_places);
    cfg.service(create_place);
    cfg.service(update_place);
    cfg.service(delete_place);
}
