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
use super::Damage;
use super::db;

type DbConnPool = Pool<SqliteConnectionManager>;

#[actix_web::get("/damage")]
async fn get_damages(pool: Data<DbConnPool>, query: web::Query<Info>) -> HttpResponse {
    let page_number = query.page.unwrap_or(1);
    let result = helpers::db_query(pool, move |conn| { 
        db::get_damages(conn, query.id.clone(), query.search.clone(), page_number) 
    }).await;

    helpers::create_return_table_object(result)
}


#[actix_web::post("/damage")]
async fn create_damage(pool: Data<DbConnPool>, damage_json: web::Json<Damage>) -> HttpResponse {
    let damage = damage_json.into_inner();

    let verification_result = damage.validate();
    if !verification_result.success {return HttpResponse::Ok().json(verification_result)};

    let result = helpers::db_query(pool, move |conn| { 
        db::create_damage(conn, damage) 
    }).await;
    helpers::create_return_status_object(result) 
}

#[actix_web::put("/damage/{id}")]
async fn update_damage(pool: Data<DbConnPool>, damage_json: web::Json<Damage>, path: Path<String>) -> HttpResponse {
    let damage = damage_json.into_inner();
    let damage_id = path.into_inner();

    let verification_result = damage.validate();
    if !verification_result.success {return HttpResponse::Ok().json(verification_result)};

    let result = helpers::db_query(pool, move |conn| { 
        db::update_damage(conn, damage_id, damage) 
    }).await;
    helpers::create_return_status_object(result)
       
}

#[actix_web::delete("/damage/{id}")]
async fn delete_damage(pool: Data<DbConnPool>, path: Path<String>) -> HttpResponse {
    let damage_id = path.into_inner();

    let result = helpers::db_query(pool, move |conn| { 
        db::delete_damage(conn, damage_id) 
    }).await;
    helpers::create_return_status_object(result)
}


pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_damages);
    cfg.service(create_damage);
    cfg.service(update_damage);
    cfg.service(delete_damage);
}
