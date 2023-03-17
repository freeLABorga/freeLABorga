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
use actix_web::{web, web::Data, HttpResponse, http::StatusCode};

use crate::helpers;
use super::db;

type DbConnPool = Pool<SqliteConnectionManager>;

#[actix_web::get("/heatmap")]
async fn get_gegenstaende(pool: Data<DbConnPool>) -> HttpResponse {
    
    let select = helpers::db_query(pool, move |conn| {
        db::get_gegenstaende(conn,1)
    }).await;

    match select {
        Ok(r)=> HttpResponse::Ok().json(r),
        Err(e)=> HttpResponse::Ok().status(StatusCode::INTERNAL_SERVER_ERROR).body(format!("{}", e))
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_gegenstaende);
}
