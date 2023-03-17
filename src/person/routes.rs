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
use super::Person;
use super::db;

type DbConnPool = Pool<SqliteConnectionManager>;


#[actix_web::get("/person")]
async fn get_persons(pool: Data<DbConnPool>, query: web::Query<Info>) -> HttpResponse {
    let page_number = query.page.unwrap_or(1);
    let result = helpers::db_query(pool, move |conn| { 
        db::get_persons(conn, query.id.clone(), query.search.clone(), page_number) 
    }).await;

    helpers::create_return_table_object(result)
}


#[actix_web::post("/person")]
async fn create_person(pool: Data<DbConnPool>, person_json: web::Json<Person>) -> HttpResponse {
    let person = person_json.into_inner();

    let verification_result = person.validate();
    if !verification_result.success {return HttpResponse::Ok().json(verification_result)};



    let result = helpers::db_query(pool, move |conn| { 
        db::create_person(conn, person) 
    }).await;
    helpers::create_return_status_object(result) 
}

#[actix_web::put("/person/{matr_nr}")]
async fn update_person(pool: Data<DbConnPool>, person_json: web::Json<Person>, path: Path<String>) -> HttpResponse {
    let person = person_json.into_inner();
    let person_id = path.into_inner();

    let verification_result = person.validate();
    if !verification_result.success {return HttpResponse::Ok().json(verification_result)};

    let result = helpers::db_query(pool, move |mut conn| { 
        db::update_person(&mut conn, person_id, person) 
    }).await;
    helpers::create_return_status_object(result)
       
}


#[actix_web::delete("/person/{matr_nr}")]
async fn delete_person(pool: Data<DbConnPool>, path: Path<String>) -> HttpResponse {
    let person_id = path.into_inner();

    let result = helpers::db_query(pool, move |conn| { 
        db::delete_person(conn, person_id) 
    }).await;
    helpers::create_return_status_object(result)
}   


#[actix_web::get("/person_to_anonymize")]
async fn get_person_to_anonymize(pool: Data<DbConnPool>) -> HttpResponse {
    let result = helpers::db_query(pool, move |conn| { 
        db::get_person_to_anonymize(conn) 
    }).await;

    helpers::create_return_table_object(result)
}

#[actix_web::delete("/person/{matr_nr}/exclude_from_anonymisation")]
async fn exclude_from_anonymisation(pool: Data<DbConnPool>, path: Path<String>) -> HttpResponse {
    let person_id = path.into_inner();

    let result = helpers::db_query(pool, move |mut conn| { 
        db::exclude_from_anonymisation(&mut conn, &person_id) 
    }).await;
    helpers::create_return_status_object(result)      
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_persons);
    cfg.service(create_person);
    cfg.service(update_person);
    cfg.service(delete_person);
    cfg.service(get_person_to_anonymize);
    cfg.service(exclude_from_anonymisation);
}
