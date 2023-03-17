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
use actix_web::{web, web::Data, web::Path, App, HttpServer, middleware, HttpResponse, http::StatusCode};
use env_logger::Env;
use tera::Tera;
use local_ip_address;
mod models;
mod helpers;

#[cfg(test)]
mod test_helpers;

mod person;
mod gegenstand;
mod lagerplatz;
mod admin;
mod lend;
mod damage;
mod suche;
mod heatmap;

const DB_FILE_PATH: &str = "db.db3";
const SERVER_PORT: u16 = 8080;
const STARTPAGE: &str = "index.html";
const ERROR_PAGE: &str  = "404.html";


/// Rendert die HTML-Templates und liefert sie aus
#[cfg(not(tarpaulin_include))]
#[actix_web::get("/{file_name}")]
async fn render_templ(data: Data<Tera>, path: Path<String>) -> HttpResponse {
    let mut name = path.into_inner();
    if name.is_empty() {
        name = STARTPAGE.to_string();
    }

    match data.render(&name, &tera::Context::new()) {
        Ok(rendered) => HttpResponse::Ok().body(rendered),
        Err(error) => match error.kind {
            tera::ErrorKind::TemplateNotFound(_) => page_not_found(data).await,
            other_error => panic!("{:?}", other_error)
        }
    }
}

/// Weiterleitung von "/" auf "/index.html" (Startseite)
#[cfg(not(tarpaulin_include))]
#[actix_web::get("/")]
async fn home_redirect() -> HttpResponse {
    HttpResponse::Ok()
        .status(StatusCode::TEMPORARY_REDIRECT)
        .append_header((
            actix_web::http::header::LOCATION,
            actix_web::http::header::HeaderValue::from_static(STARTPAGE),
        )).finish()
}


/// Fehlerseite 404 anzeigen, wenn angefragte Seite nicht gefunden wurde
#[cfg(not(tarpaulin_include))]
async fn page_not_found(data: Data<Tera>) -> HttpResponse {
    let rendered = data.render(ERROR_PAGE, &tera::Context::new()).unwrap_or("Page not found".to_string());
    HttpResponse::Ok().status(StatusCode::NOT_FOUND).body(rendered)
}


/// Startet den Webserver und die Datenbankanbindung und richtet das Routing ein
#[cfg(not(tarpaulin_include))]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    let manager = SqliteConnectionManager::file(DB_FILE_PATH);
    let pool = Pool::new(manager).expect("Die Datenbankdatei konnte nicht geöffnet werden");

    helpers::init_tables(pool.get().unwrap()).expect("Error while create db tables");

    let local_ip = local_ip_address::local_ip();
    println!("----------------------------------------------------------------------------------------------");
    if let Ok(local_ip) = local_ip {
        println!("   Weboberfläche unter folgender URL erreichbar: http://{:?}:8080/", local_ip);
    } else {
        println!("   Weboberfläche unter folgender URL erreichbar: http://localhost:8080/");
    }
    println!("----------------------------------------------------------------------------------------------\n");

    HttpServer::new(move || {
        let tera = Tera::new("templates/**/*.html").unwrap();

        App::new()
            .app_data(Data::new(pool.clone()))              // Rusqlite-DB-Connection-Pool
            .app_data(Data::new(tera.clone()))              // zum Rendern von HTML-Vorlagen

            .service(web::scope("/api")                     // API (über JSON-Dokumente)
                .configure(person::init_routes)      
                .configure(gegenstand::init_routes)
                .configure(lagerplatz::init_routes)
                .configure(lend::init_routes)
                .configure(damage::init_routes)
                .configure(admin::init_routes)
                .configure(suche::init_routes)
                .configure(heatmap::init_routes)
            )

            .service(render_templ)                              // HTML-Vorlagen rendern
            .service(home_redirect)                             // Bei Aufruf von "/" nach "/index.html" weiterleiten
            .service(actix_files::Files::new("/", "static"))    // Statische Dateien (css, js) ausliefern
            .default_service(web::route().to(page_not_found))   // Fehlerseite 404
            .wrap(middleware::Logger::default())                // Logausgaben in der Konsole
    })
    .bind(("0.0.0.0", SERVER_PORT))?
    .run()
    .await
}
