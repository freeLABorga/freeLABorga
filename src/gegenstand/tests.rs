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
    use super::super::{routes, Gegenstand, GegenstandUebersicht};

    #[actix_web::test]
    async fn get_all_items() {
        let expected_result = MainJson {
            page_number: 1,  
            pages_total: 1,  
            items_on_page: 3,
            items_total: 3,  
            data: vec![
                Gegenstand {
                    id: String::from("olimexino"),
                    name: String::from("OLIMEXINO-STM32"),
                    serialnumber: String::from("9Q9Q-Z3YF-PVEJ-4AD5"),
                    price: 1995 as f64 / 100.0,
                    buydate: String::from("2022-06-24"),
                    inventoried: Some(true),
                    available: Some(true),
                    place: String::from("Büro"),
                    categories: vec![String::from("Mikrocontroller"), String::from("Schwarz")],
                },
                Gegenstand {
                    id: String::from("pine_phone_a"),
                    name: String::from("PinePhone Pro Explorer Edition"),
                    serialnumber: String::from("N9TT-9G0A-B7FQ-RANC"),
                    price: 42350 as f64 / 100.0,
                    buydate: String::from("2020-10-04"),
                    inventoried: Some(true),
                    available: Some(false),
                    place: String::from("Labor"),
                    categories: vec![String::from("Linux"), String::from("Schwarz"), String::from("Smartphone")],
                },
                Gegenstand {
                    id: String::from("pine_phone_b"),
                    name: String::from("PinePhone Pro Explorer Edition"),
                    serialnumber: String::from("HEY9-EFXB-UH56-D465"),
                    price: 42350 as f64 / 100.0,
                    buydate: String::from("2020-10-04"),
                    inventoried: Some(true),
                    available: Some(false),
                    place: String::from("Labor"),
                    categories: vec![String::from("Linux"), String::from("Schwarz"), String::from("Smartphone")],
                },
                
            ],
        };


        let app = test_helpers::init_api_test(routes::init_routes).await;
        let req = TestRequest::get().uri("/api/gegenstand").to_request();
        let resp: MainJson<Gegenstand> = test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp, expected_result, "Failed to get lab items");
    }

    #[actix_web::test]
    async fn get_items_overview() {
        let expected_result = MainJson {
            page_number: 1,  
            pages_total: 1,  
            items_on_page: 2,
            items_total: 2,  
            data: vec![
                GegenstandUebersicht {
                    name: String::from("OLIMEXINO-STM32"),
                    categories: vec![String::from("Mikrocontroller"), String::from("Schwarz")],
                    total: 1,
                    available: 1,
                },
                GegenstandUebersicht {
                    name: String::from("PinePhone Pro Explorer Edition"),
                    categories: vec![String::from("Linux"), String::from("Schwarz"), String::from("Smartphone")],
                    total: 2,
                    available: 0,
                },
                
            ],
        };


        let app = test_helpers::init_api_test(routes::init_routes).await;
        let req = TestRequest::get().uri("/api/gegenstand_uebersicht").to_request();
        let resp: MainJson<GegenstandUebersicht> = test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp, expected_result, "Failed to get lab items overview");
    }

    #[actix_web::test]
    async fn search_lab_item() {
        let expected_result = MainJson {
            page_number: 1,  
            pages_total: 1,  
            items_on_page: 1,
            items_total: 1,  
            data: vec![
                Gegenstand {
                    id: String::from("pine_phone_b"),
                    name: String::from("PinePhone Pro Explorer Edition"),
                    serialnumber: String::from("HEY9-EFXB-UH56-D465"),
                    price: 42350 as f64 / 100.0,
                    buydate: String::from("2020-10-04"),
                    inventoried: Some(true),
                    available: Some(false),
                    place: String::from("Labor"),
                    categories: vec![String::from("Linux"),String::from("Schwarz"),  String::from("Smartphone")],
                },
            ],
        };

        let app = test_helpers::init_api_test(routes::init_routes).await;
        let req = TestRequest::get().uri("/api/gegenstand?search=pine_phone_b").to_request();
        let resp: MainJson<Gegenstand> = test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp, expected_result, "Failed to get searched items");
    }

    #[actix_web::test]
    async fn insert_item_ok() {
        let item_to_insert = Gegenstand {
            id: String::from("test_item"),
            name: String::from("Testname"),
            serialnumber: String::from("SERIALNUMBER"),
            price: 1000 as f64 / 100.0,
            buydate: String::from("2022-11-14"),
            inventoried: Some(true),
            available: Some(true),
            place: String::from("Labor"),
            categories: vec![String::from("Smartphone"), String::from("Testkategorie"), String::from("teuer")],
        };

        let app = test_helpers::init_api_test(routes::init_routes).await;
        let req = TestRequest::post().uri("/api/gegenstand").set_json(&item_to_insert).to_request();
        let resp: OperationResult = test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp.success, true, "Failed to insert item: {}", &resp.message);


        let url = format!("/api/gegenstand?id={}", item_to_insert.id);
        let req2 = TestRequest::get().uri(&url).to_request();
        let resp: MainJson<Gegenstand> = test::call_and_read_body_json(&app, req2).await;

        assert_eq!(resp.data.first(), Some(&item_to_insert), "Failed find item after inserting");
    }

    #[actix_web::test]
    async fn insert_item_fail() {
        let item_empty_fields = Gegenstand {
            id: String::from(""),
            name: String::from(""),
            serialnumber: String::from(""),
            price: 0 as f64 / 100.0,
            buydate: String::from(""),
            place: String::from(""),
            categories: vec![],
            available: None,
            inventoried: None,
        };

        let item_id_existing = Gegenstand {
            id: String::from("pine_phone_b"),
            name: String::from("Testname"),
            serialnumber: String::from("SERIALNUMBER"),
            price: 1000 as f64 / 100.0,
            buydate: String::from("2022-11-14"),
            inventoried: Some(true),
            available: Some(true),
            place: String::from("Labor"),
            categories: vec![String::from("Testkategorie"), String::from("Smartphone"), String::from("teuer")],
        };
        let app = test_helpers::init_api_test(routes::init_routes).await;

        let req = TestRequest::post().uri("/api/gegenstand").set_json(&item_empty_fields).to_request();
        let resp: OperationResult = test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp.success, false, "Error expected");
        assert_eq!(resp.message, "Empty fields: name, id, place, buydate".to_string(), "Wrong error message: {}", &resp.message);

        let req = TestRequest::post().uri("/api/gegenstand").set_json(&item_id_existing).to_request();
        let resp: OperationResult = test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp.success, false, "Error expected");
        assert_eq!(resp.message, "ID bereits vorhanden".to_string(), "Wrong error message: {}", &resp.message);
    }

    #[actix_web::test]
    async fn update_item_ok() {
        let old_item_id = "pine_phone_b";
        let item_to_update = Gegenstand {
            id: String::from("pine_phone_b2"),
            name: String::from("PinePhone Pro Explorer Editionnnnnn"),
            serialnumber: String::from("TESTNUMMER"),
            price: 12345 as f64 / 100.0,
            buydate: String::from("2020-01-01"),
            //Inventoried und available wird nicht geändert, da diese Werte von Nutzer nicht manuell verändert werden dürfen
            inventoried: Some(true),
            available: Some(false),
            place: String::from("Testort"),
            categories: vec![String::from("cool"), String::from("neu"), String::from("toll")],
        };

        let app = test_helpers::init_api_test(routes::init_routes).await;
        let url = format!("/api/gegenstand/{}", &old_item_id);
        let req = TestRequest::put().uri(&url).set_json(&item_to_update).to_request();
        let resp: OperationResult = test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp.success, true, "Failed to update item: {}", &resp.message);


        let url = format!("/api/gegenstand?id={}", item_to_update.id);
        let req2 = TestRequest::get().uri(&url).to_request();
        let resp: MainJson<Gegenstand> = test::call_and_read_body_json(&app, req2).await;

        assert_eq!(resp.data.first(), Some(&item_to_update), "Failed find item after updating");

        let url = format!("/api/gegenstand?id={}", &old_item_id);
        let req3= TestRequest::get().uri(&url).to_request();
        let resp: MainJson<Gegenstand> = test::call_and_read_body_json(&app, req3).await;

        assert!(resp.data.is_empty(), "Still found old item after updating");
    }

    #[actix_web::test]
    async fn delete_item_ok() {
        let old_item_id = "pine_phone_b";

        let app = test_helpers::init_api_test(routes::init_routes).await;
        let url = format!("/api/gegenstand/{}", &old_item_id);
        let req = TestRequest::delete().uri(&url).to_request();
        let resp: OperationResult = test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp.success, true, "Failed to delete item: {}", &resp.message);

        let url = format!("/api/gegenstand?id={}", &old_item_id);
        let req3= TestRequest::get().uri(&url).to_request();
        let resp: MainJson<Gegenstand> = test::call_and_read_body_json(&app, req3).await;

        assert!(resp.data.is_empty(), "Still found old item after deleting");
    }

    #[actix_web::test]
    async fn inventory_out_item_ok() {
        let item_id = "pine_phone_b";
        
        let item_ninventory = Gegenstand {
            id: String::from("pine_phone_b"),
            name: String::from("PinePhone Pro Explorer Edition"),
            serialnumber: String::from("HEY9-EFXB-UH56-D465"),
            price: 42350 as f64 / 100.0,
            buydate: String::from("2020-10-04"),
            inventoried: Some(false),
            available: Some(false),
            place: String::from("Labor"),
            categories: vec![String::from("Linux"), String::from("Schwarz"), String::from("Smartphone")],
        };
        let app = test_helpers::init_api_test(routes::init_routes).await;
        let url = format!("/api/gegenstand/ninventory/{}", &item_id);
        let req = TestRequest::delete().uri(&url).to_request();
        let resp: OperationResult = test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp.success, true, "Failed to inventory item out: {}", &resp.message);

        let url = format!("/api/gegenstand?id={}", item_id);
        let req2 = TestRequest::get().uri(&url).to_request();
        let resp: MainJson<Gegenstand> = test::call_and_read_body_json(&app, req2).await;

        assert_eq!(resp.data.first(), Some(&item_ninventory), "Item still is inventoried");
    }


}
