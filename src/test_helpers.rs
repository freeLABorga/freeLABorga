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

use actix_http::Request;
use actix_web::{test, App, web::{Data, self, ServiceConfig}, dev::{Service, ServiceResponse}};
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use std::fs;

const PATH: &str = "test_data.sql";

pub async fn init_api_test<F>(cfg_fn: F)  -> impl Service<Request, Response = ServiceResponse, Error = actix_web::Error>
where
    F: FnOnce(&mut ServiceConfig),
{
    let manager = SqliteConnectionManager::memory();
    let pool = Pool::new(manager).expect("Die Datenbankdatei konnte nicht geöffnet werden");
    
    init_test_tables(pool.get().unwrap());
    
    test::init_service(
        App::new()
            .app_data(Data::new(pool.clone()))

            .service(web::scope("/api")
                .configure(cfg_fn)
            )
    ).await
}

fn init_test_tables(conn: PooledConnection<SqliteConnectionManager>) {
    let sql = fs::read_to_string(PATH).expect("Unable to read SQL file.");
    conn.execute_batch(&sql).expect("Uable to fill tables with predefined test data.");
}
