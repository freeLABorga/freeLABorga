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

#[cfg(test)]
mod tests {
    use actix_web::test::{self, TestRequest};

    use crate::models::{MainJson, OperationResult};
    use crate::test_helpers;
    use super::super::{routes, Lend};

    #[actix_web::test]
    async fn get_all_lends() {
        let expected_result = MainJson {
            page_number: 1,  
            pages_total: 1,  
            items_on_page: 4,
            items_total: 4,  
            data: vec![
                Lend {
                    id_person: Some(String::from("ABC1234")),
                    id_lab_item: Some(String::from("pine_phone_a")),
                    lend_date: Some(String::from("2022-05-03")),
                    planned_return_date: Some(String::from("2022-07-03")),
                    actual_return_date: Some(String::from("2022-07-11")),
                    id: Some(2),
                    firstname: Some(String::from("Erika")),
                    lastname: Some(String::from("Musterfrau")),
                },
                Lend {
                    id_person: Some(String::from("ABC1234")),
                    id_lab_item: Some(String::from("pine_phone_b")),
                    lend_date: Some(String::from("2022-08-29")),
                    planned_return_date: Some(String::from("2022-09-29")),
                    actual_return_date: Some(String::from("2022-09-18")),
                    id: Some(4),
                    firstname: Some(String::from("Erika")),
                    lastname: Some(String::from("Musterfrau")),
                },
                Lend {
                    id_person: Some(String::from("1234567")),
                    id_lab_item: Some(String::from("pine_phone_a")),
                    lend_date: Some(String::from("2022-10-18")),
                    planned_return_date: Some(String::from("2022-12-18")),
                    actual_return_date: None,
                    id: Some(1),
                    firstname: Some(String::from("Max")),
                    lastname: Some(String::from("Mustermann")),
                },
                Lend {
                    id_person: Some(String::from("1234567")),
                    id_lab_item: Some(String::from("pine_phone_b")),
                    lend_date: Some(String::from("2022-11-25")),
                    planned_return_date: Some(String::from("2022-12-25")),
                    actual_return_date: None,
                    id: Some(3),
                    firstname: Some(String::from("Max")),
                    lastname: Some(String::from("Mustermann")),
                }
                
            ],
        };


        let app = test_helpers::init_api_test(routes::init_routes).await;
        let req = TestRequest::get().uri("/api/lend").to_request();
        let resp: MainJson<Lend> = test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp, expected_result, "Failed to get the lends");
    }

    #[actix_web::test]
    async fn get_individual_lend() {
        let expected_result = MainJson {
            page_number: 1,  
            pages_total: 1,  
            items_on_page: 2,
            items_total: 2,  
            data: vec![
                Lend {
                    id_person: Some(String::from("1234567")),
                    id_lab_item: Some(String::from("pine_phone_a")),
                    lend_date: Some(String::from("2022-10-18")),
                    planned_return_date: Some(String::from("2022-12-18")),
                    actual_return_date: None,
                    id: Some(1),
                    firstname: Some(String::from("Max")),
                    lastname: Some(String::from("Mustermann")),
                },
                Lend {
                    id_person: Some(String::from("ABC1234")),
                    id_lab_item: Some(String::from("pine_phone_a")),
                    lend_date: Some(String::from("2022-05-03")),
                    planned_return_date: Some(String::from("2022-07-03")),
                    actual_return_date: Some(String::from("2022-07-11")),
                    id: Some(2),
                    firstname: Some(String::from("Erika")),
                    lastname: Some(String::from("Musterfrau")),
                }
            ],
        };


        let app = test_helpers::init_api_test(routes::init_routes).await;
        let req = TestRequest::get().uri("/api/lend?id=pine_phone_a").to_request();
        let resp: MainJson<Lend> = test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp, expected_result, "Failed to get the lends");
    }

    #[actix_web::test]
    async fn get_individual_lend_fail() {
        let expected_result = MainJson {
            page_number: 1,  
            pages_total: 1,  
            items_on_page: 0,
            items_total: 0,  
            data: vec![],   //should be empty, because theres no lab item with this id
        };

        let app = test_helpers::init_api_test(routes::init_routes).await;
        let req = TestRequest::get().uri("/api/lend?id=item_doesnt_exist").to_request();
        let resp: MainJson<Lend> = test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp, expected_result, "Failed to get the lends");
    }

    #[actix_web::test]
    async fn create_lend() {
        let lend_to_create = Lend {
                id_person: Some(String::from("ABC1234")),
                id_lab_item: Some(String::from("olimexino")),
                lend_date: Some(String::from("2023-01-11")),
                planned_return_date: Some(String::from("2023-02-01")),
                actual_return_date: None,
                id: None,
                firstname:None,
                lastname: None,
        };
        let expected_result = MainJson {
            page_number: 1,  
            pages_total: 1,  
            items_on_page: 1,
            items_total: 1,  
            data: vec![Lend {
                id_person: Some(String::from("ABC1234")),
                id_lab_item: Some(String::from("olimexino")),
                lend_date: Some(String::from("2023-01-11")),
                planned_return_date: Some(String::from("2023-02-01")),
                actual_return_date: None,
                id: Some(5),
                firstname:Some(String::from("Erika")),
                lastname: Some(String::from("Musterfrau")),
        }],
        };

        let app = test_helpers::init_api_test(routes::init_routes).await;
        let req = TestRequest::post().uri("/api/lend/olimexino").set_json(&lend_to_create).to_request();
        let resp: OperationResult = test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp.success, true, "Failed to create lend: {}", &resp.message);

        let req2 = TestRequest::get().uri("/api/lend?id=olimexino").to_request();
        let resp: MainJson<Lend> = test::call_and_read_body_json(&app, req2).await;

        assert_eq!(resp, expected_result, "Failed find item after inserting");

    }

    #[actix_web::test]
    async fn create_lend_with_non_existent_person() {
        let lend_to_create = Lend {
                id_person: Some(String::from("IdontExist")),
                id_lab_item: Some(String::from("olimexino")),
                lend_date: Some(String::from("2023-01-11")),
                planned_return_date: Some(String::from("2023-02-01")),
                actual_return_date: None,
                id: None,
                firstname:None,
                lastname: None,
        };

        let app = test_helpers::init_api_test(routes::init_routes).await;
        let req = TestRequest::post().uri("/api/lend/olimexino").set_json(&lend_to_create).to_request();
        let resp: OperationResult = test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp.success, false, "Failed to create lend: {}", &resp.message);

    }

    #[actix_web::test]
    async fn delete_lend() {
        let old_lend_id = "4";
        let expected_result = MainJson {
            page_number: 1,  
            pages_total: 1,  
            items_on_page: 1,
            items_total: 1,  
            data: vec![Lend {   //only 1 lend expected, beacause Lend with id 4 deleted
                id_person: Some(String::from("1234567")),
                id_lab_item: Some(String::from("pine_phone_b")),
                lend_date: Some(String::from("2022-11-25")),
                planned_return_date: Some(String::from("2022-12-25")),
                actual_return_date: None,
                id: Some(3),
                firstname: Some(String::from("Max")),
                lastname: Some(String::from("Mustermann")),
            }],
        };

        let app = test_helpers::init_api_test(routes::init_routes).await;
        let url = format!("/api/lend/{}", &old_lend_id);
        let req = TestRequest::delete().uri(&url).to_request();
        let resp: OperationResult = test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp.success, true, "Failed to delete lend: {}", &resp.message);

        let url = format!("/api/lend?id={}", "pine_phone_b");
        let req3= TestRequest::get().uri(&url).to_request();
        let resp: MainJson<Lend> = test::call_and_read_body_json(&app, req3).await;

        assert_eq!(resp, expected_result, "Still found old lend after deleting");
    }

    #[actix_web::test]
    async fn update_item_ok() {
        let lend_to_update = Lend {
            id_person: Some(String::from("1234567")),
            id_lab_item: Some(String::from("pine_phone_a")),
            lend_date: Some(String::from("2022-10-18")),
            planned_return_date: Some(String::from("2022-12-18")),
            actual_return_date: Some(String::from("2022-12-19")),
            id: Some(1),
            firstname: Some(String::from("Max")),
            lastname: Some(String::from("Mustermann")),
        };

        let app = test_helpers::init_api_test(routes::init_routes).await;
        let req = TestRequest::put().uri("/api/lend/pine_phone_a").set_json(&lend_to_update).to_request();
        let resp: OperationResult = test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp.success, true, "Failed to update lend: {}", &resp.message);

        let req2 = TestRequest::get().uri("/api/lend?id=pine_phone_a").to_request();
        let resp: MainJson<Lend> = test::call_and_read_body_json(&app, req2).await;

        assert_eq!(resp.data.first(), Some(&lend_to_update), "Failed find item after updating");
    }
}
