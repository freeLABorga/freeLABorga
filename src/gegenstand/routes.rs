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
use actix_web::{web, web::Data,web::Path, HttpResponse, http::StatusCode};

use crate::helpers;
use crate::models::Info;
use crate::gegenstand::Gegenstand;
use crate::gegenstand::db;

type DbConnPool = Pool<SqliteConnectionManager>;

#[actix_web::get("/gegenstand")]
async fn get_gegenstaende(pool: Data<DbConnPool>, query: web::Query<Info>) -> HttpResponse {
    let page_number = query.page.unwrap_or(1);
    let select = helpers::db_query(pool, move |conn| {
        db::get_gegenstaende(conn, query.id.clone(), query.search.clone(),query.item_name.clone(), query.cat.clone(), query.place.clone(), page_number)
    }).await;

    match select {
        Ok(r)=> HttpResponse::Ok().json(r),
        Err(e)=> HttpResponse::Ok().status(StatusCode::INTERNAL_SERVER_ERROR).body(format!("{}", e))
    }
}

#[actix_web::post("/gegenstand")]
async fn create_gegenstand(pool: Data<DbConnPool>, gegenstand_json: web::Json<Gegenstand>) -> HttpResponse {
    //First add default values / Format json inputs
    let mut gegenstand =  gegenstand_json.into_inner();
    gegenstand.available = Some(true);
    gegenstand.inventoried = Some(true);
    let verification_result = gegenstand.validate();
    if !verification_result.success {
        return HttpResponse::Ok().json(verification_result)
    }

    let result = helpers::db_query(pool, move |conn| {
        db::create_gegenstand(conn, gegenstand)
    }).await;
    helpers::create_return_status_object(result)
}


#[actix_web::put("/gegenstand/{id}")]
async fn update_gegenstand(pool: Data<DbConnPool>, gegenstand_json: web::Json<Gegenstand>, path: Path<String>) -> HttpResponse {
    let gegenstand = gegenstand_json.into_inner();
    let gegenstand_id = path.into_inner();

    let verification_result = gegenstand.validate();
    if !verification_result.success {
        return HttpResponse::Ok().json(verification_result)
    }

    let result = helpers::db_query(pool, move |conn| {
        db::update_gegenstand(conn, gegenstand_id, gegenstand)
    }).await;
    helpers::create_return_status_object(result)
}

//Für das ausinventarisieren eines Gegenstands mit einer ID
#[actix_web::delete("/gegenstand/ninventory/{id}")]
async fn not_inventory_gegenstand(pool: Data<DbConnPool>, path: Path<String>) -> HttpResponse {
    let gegenstand_id = path.into_inner();

    let result = helpers::db_query(pool, move |conn| {
        db::not_inventory_gegenstand(conn, gegenstand_id)
    }).await;
    helpers::create_return_status_object(result)
}

//Für das inventarisieren eines Gegenstands mit einer ID
#[actix_web::delete("/gegenstand/inventory/{id}")]
async fn inventory_gegenstand(pool: Data<DbConnPool>, path: Path<String>) -> HttpResponse {
    let gegenstand_id = path.into_inner();

    let result = helpers::db_query(pool, move |conn| {
        db::inventory_gegenstand(conn, gegenstand_id)
    }).await;
    helpers::create_return_status_object(result)
}


#[actix_web::delete("/gegenstand/{id}")]
async fn delete_gegenstand(pool: Data<DbConnPool>, path: Path<String>) -> HttpResponse{
    let gegenstand_id = path.into_inner();

    let result = helpers::db_query(pool, move |conn| {
        db::delete_gegenstand(conn,gegenstand_id)
    }).await;
    helpers::create_return_status_object(result)
}

#[actix_web::get("/gegenstand_uebersicht")]
async fn get_things_overview(pool: Data<DbConnPool>, query: web::Query<Info>) -> HttpResponse {
    let page_number = query.page.unwrap_or(1);
    let result = helpers::db_query(pool, move |conn| { 
        db::get_things_overview(conn, page_number) 
    }).await;

    helpers::create_return_table_object(result)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_gegenstaende);
    cfg.service(create_gegenstand);
    cfg.service(update_gegenstand);
    cfg.service(delete_gegenstand);
    cfg.service(get_things_overview);
    cfg.service(inventory_gegenstand);
    cfg.service(not_inventory_gegenstand);
}
