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
use super::Admin;
use super::db;

type DbConnPool = Pool<SqliteConnectionManager>;


#[actix_web::get("/admin")]
async fn get_admin_texts(pool: Data<DbConnPool>) -> HttpResponse {
    let result = helpers::db_query(pool, move |conn| { 
        db::get_admin_texts(conn) 
    }).await;

    helpers::create_return_table_object(result)
}


#[actix_web::put("/admin")]
async fn update_admin_texts(pool: Data<DbConnPool>, admin_texts_json: web::Json<Admin>) -> HttpResponse {
    let admin_texts = admin_texts_json.into_inner();

    let result = helpers::db_query(pool, move |conn| { 
        db::update_admin_texts(conn, admin_texts)
    }).await;
    helpers::create_return_status_object(result)
       
}


#[actix_web::get("/impressum")]
async fn imprint(pool: Data<DbConnPool>) -> HttpResponse {
    imprint_or_privacy(pool, "imprint_text".to_string(), "imprint_html".to_string()).await

}

#[actix_web::get("/datenschutz")]
async fn privacy(pool: Data<DbConnPool>) -> HttpResponse {
    imprint_or_privacy(pool, "privacy_text".to_string(), "privacy_html".to_string()).await
}

async fn imprint_or_privacy(pool: Data<DbConnPool>, key: String, is_html_key: String) -> HttpResponse {
    let result = helpers::db_query(pool, move |conn| { 
        let mut text = db::get_config_texts(conn, &key)?;
        if db::get_config_texts(conn, &is_html_key)? == "0".to_string() {
            text = escape_html(text);
        }
        Ok(text)
    }).await.unwrap_or("Fehler beim Laden des Textes.".to_string());

    HttpResponse::Ok().body(result)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_admin_texts);
    cfg.service(update_admin_texts);
    cfg.service(imprint);
    cfg.service(privacy);
}


pub fn escape_html(input_text: String) -> String {
    let mut output_text = input_text;
    let replace_list = [
        ("&", "&amp;"),
        ("<", "&lt;"),
        (">", "&gt;"),
        ("'", "&apos;"),
        ("\n", "<br>"),
        ("\r", ""),
        ("\"", "&quot;"),
    ];


    for i in replace_list {
        output_text = output_text.replace(i.0, i.1);
    }
    output_text
}
