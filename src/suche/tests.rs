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

// Hier kommen die Automatisierten Tests hin
#[cfg(test)]
mod tests {
    use actix_web::test::{self, TestRequest};

    use crate::test_helpers;
    use super::super::{routes, Suggestion, SearchItem, CategoriesPlaces};

    #[actix_web::test]
    async fn search_things_persons() {
        let expected_result = Suggestion {
            things: vec![
                SearchItem {
                    id: "pine_phone_b".to_string(),
                    name: "PinePhone Pro Explorer Edition".to_string(),
                },
            ],
            persons: vec![
                SearchItem {
                    id: "ABC1234".to_string(),
                    name: "Musterfrau, Erika".to_string(),
                },
            ],
        };

        let app = test_helpers::init_api_test(routes::init_routes).await;
        let req = TestRequest::get().uri("/api/schnellsuche?search=b").to_request();
        let resp: Suggestion = test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp, expected_result, "Failed to get search items");
    }

    #[actix_web::test]
    async fn search_categories_places() {
        let expected_result = CategoriesPlaces {
            categories: vec![
                "Smartphone".to_string(),
                "Mikrocontroller".to_string(),
                "Linux".to_string(),
                "Schwarz".to_string(),
            ],
            place: vec![
                "Labor".to_string(),
                "Büro".to_string(),
            ],
        };

        let app = test_helpers::init_api_test(routes::init_routes).await;
        let req = TestRequest::get().uri("/api/kategorie_lagerplatz").to_request();
        let resp: CategoriesPlaces = test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp, expected_result, "Failed to get categories and places");
    }

}
