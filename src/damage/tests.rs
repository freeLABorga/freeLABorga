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

    use crate::models::{MainJson, OperationResult};
    use crate::test_helpers;
    use super::super::{routes, Damage};

    #[actix_web::test]
    async fn get_all_damages() {
        let expected_result = MainJson {
            page_number: 1,  
            pages_total: 1,  
            items_on_page: 4,
            items_total: 4,  
            data: vec![
                Damage {
                    id: Some(10),
                    lab_item_id: "olimexino".to_string(),
                    date: "2022-05-19".to_string(),
                    description: "Pin abgebrochen".to_string(),
                    repaired: true
                },
                Damage {
                    id: Some(2),
                    lab_item_id: "pine_phone_a".to_string(),
                    date: "2022-11-25".to_string(),
                    description: "Kratzer am Gehäuse".to_string(),
                    repaired: false
                },
                Damage {
                    id: Some(3),
                    lab_item_id: "pine_phone_a".to_string(),
                    date: "2022-11-26".to_string(),
                    description: "Akkuschaden".to_string(),
                    repaired: true
                },
                Damage {
                    id: Some(1),
                    lab_item_id: "pine_phone_a".to_string(),
                    date: "2022-12-11".to_string(),
                    description: "Displayschaden".to_string(),
                    repaired: true
                }
            ],
        };


        let app = test_helpers::init_api_test(routes::init_routes).await;
        let req = TestRequest::get().uri("/api/damage").to_request();
        let resp: MainJson<Damage> = test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp, expected_result, "Failed to get damages");
    }

    #[actix_web::test]
    async fn insert_damage_ok() {
        let item_to_insert = Damage {
            id: Some(11 as usize),
            lab_item_id: "test_id".to_string(),
            date: "2023-01-19".to_string(),
            description: "Testfall".to_string(),
            repaired: false
        };

        let app = test_helpers::init_api_test(routes::init_routes).await;
        let req = TestRequest::post().uri("/api/damage").set_json(&item_to_insert).to_request();
        let resp: OperationResult = test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp.success, true, "Failed to insert damage: {}", &resp.message);


        let url = format!("/api/damage?id={:?}", item_to_insert.id.unwrap());
        let req2 = TestRequest::get().uri(&url).to_request();
        let resp: MainJson<Damage> = test::call_and_read_body_json(&app, req2).await;

        assert_eq!(resp.data.first(), Some(&item_to_insert), "Failed find damage after inserting");
    }

    #[actix_web::test]
    async fn insert_damage_fail() {
        let damage_empty_fields = Damage {
            id: None,
            lab_item_id: "".to_string(),
            date: "".to_string(),
            description: "".to_string(),
            repaired: false
        };
        let app = test_helpers::init_api_test(routes::init_routes).await;

        let req = TestRequest::post().uri("/api/damage").set_json(&damage_empty_fields).to_request();
        let resp: OperationResult = test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp.success, false, "Error expected");
        assert_eq!(resp.message, "Empty fields: description, date, idLabItem".to_string(), "Wrong error message: {}", &resp.message);
    }

    #[actix_web::test]
    async fn update_damage_ok() {
        let old_damage_id = 1;
        let damage_to_update = Damage {
            id: Some(1 as usize),
            lab_item_id: "moto_a2".to_string(),
            date: "2022-12-12".to_string(),
            description: "Displayschaden".to_string(),
            repaired: true
        };

       let app = test_helpers::init_api_test(routes::init_routes).await;
        let url = format!("/api/damage/{}", &old_damage_id);
        let req = TestRequest::put().uri(&url).set_json(&damage_to_update).to_request();
        let resp: OperationResult = test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp.success, true, "Failed to update damage: {}", &resp.message);


        let url = format!("/api/damage?id={:?}", damage_to_update.id.unwrap());
        let req2 = TestRequest::get().uri(&url).to_request();
        let resp: MainJson<Damage> = test::call_and_read_body_json(&app, req2).await;

        assert_eq!(resp.data.first(), Some(&damage_to_update), "Failed to find damage after updating");

    }

    #[actix_web::test]
    async fn delete_damage_ok() {
        let old_damage_id = Some(4 as usize);

        let app = test_helpers::init_api_test(routes::init_routes).await;
        let url = format!("/api/damage/{:?}", &old_damage_id);
        let req = TestRequest::delete().uri(&url).to_request();
        let resp: OperationResult = test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp.success, true, "Failed to delete damage: {}", &resp.message);

        let url = format!("/api/damage?id={:?}", &old_damage_id);
        let req3= TestRequest::get().uri(&url).to_request();
        let resp: MainJson<Damage> = test::call_and_read_body_json(&app, req3).await;

        assert!(resp.data.is_empty(), "Still found old damage after deleting");
    }
}
