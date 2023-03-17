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

    use super::super::{routes, Place};
    use crate::models::{MainJson, OperationResult};
    use crate::test_helpers;

    #[actix_web::test]
    async fn get_all_places() {
        let expected_result = MainJson {
            page_number: 1,
            pages_total: 1,
            items_on_page: 2,
            items_total: 2,
            data: vec![
                Place {
                    id: Some(1),
                    name: "Labor".to_string(),
                    number: Some(2),
                },
                Place {
                    id: Some(2),
                    name: "Büro".to_string(),
                    number: Some(1),
                },
            ],
        };

        let app = test_helpers::init_api_test(routes::init_routes).await;
        let req = TestRequest::get().uri("/api/lagerplatz").to_request();
        let resp: MainJson<Place> = test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp, expected_result, "Failed to get places");
    }


    #[actix_web::test]
    async fn insert_place_ok() {
        let place_to_insert = Place {
            id: Some(3),
            name: "Testplatz".to_string(),
            number: Some(0),
        };

        let app = test_helpers::init_api_test(routes::init_routes).await;
        let req = TestRequest::post().uri("/api/lagerplatz").set_json(&place_to_insert).to_request();
        let resp: OperationResult = test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp.success, true, "Failed to insert place: {}", &resp.message);

        let url = format!("/api/lagerplatz?id={}", place_to_insert.id.unwrap());
        let req2 = TestRequest::get().uri(&url).to_request();
        let resp: MainJson<Place> = test::call_and_read_body_json(&app, req2).await;

        assert_eq!(resp.data.first(), Some(&place_to_insert), "Failed find place after inserting");
    }

    #[actix_web::test]
    async fn insert_place_fail() {
        let place_to_insert = Place {
            id: None,
            name: "".to_string(),
            number: None,
        };

        let app = test_helpers::init_api_test(routes::init_routes).await;
        let req = TestRequest::post().uri("/api/lagerplatz").set_json(&place_to_insert).to_request();
        let resp: OperationResult = test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp.success, false, "Error expected: {}", &resp.message);
        assert_eq!(resp.message, "Empty fields: name".to_string(), "Wrong error message: {}", &resp.message);
    }


    #[actix_web::test]
    async fn update_place_ok() {
        let old_place_id = 1;
        let place_to_update = Place {
            id: Some(1),
            name: "Testplatz".to_string(),
            number: Some(2),
        };

        let app = test_helpers::init_api_test(routes::init_routes).await;
        let url = format!("/api/lagerplatz/{}", &old_place_id);
        let req = TestRequest::put().uri(&url).set_json(&place_to_update).to_request();
        let resp: OperationResult = test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp.success, true, "Failed to update place: {}", &resp.message);

        let url = format!("/api/lagerplatz?id={}", place_to_update.id.unwrap());
        let req2 = TestRequest::get().uri(&url).to_request();
        let resp: MainJson<Place> = test::call_and_read_body_json(&app, req2).await;

        assert_eq!(resp.data.first(), Some(&place_to_update), "Failed find place after updating");
    }

    #[actix_web::test]
    async fn delete_person_ok() {
        let old_person_id = 1;

        let app = test_helpers::init_api_test(routes::init_routes).await;
        let url = format!("/api/lagerplatz/{}", &old_person_id);
        let req = TestRequest::delete().uri(&url).to_request();
        let resp: OperationResult = test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp.success, true, "Failed to delete place: {}", &resp.message);

        let url = format!("/api/lagerplatz?id={}", &old_person_id);
        let req3= TestRequest::get().uri(&url).to_request();
        let resp: MainJson<Place> = test::call_and_read_body_json(&app, req3).await;

        assert!(resp.data.is_empty(), "Still found old place after deleting");
    }
}
