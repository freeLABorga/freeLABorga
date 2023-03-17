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
    use super::super::{routes, Person, AnonymizePerson};

    #[actix_web::test]
    async fn get_all_persons() {
        let expected_result = MainJson {
            page_number: 1,  
            pages_total: 1,  
            items_on_page: 2,
            items_total: 2,  
            data: vec![
                Person {
                    matr_nr: "ABC1234".to_string(),
                    lastname: "Musterfrau".to_string(),
                    firstname: "Erika".to_string(),
                    email: "erika.musterfrau@study.thws.de".to_string(),
                },
                Person {
                    matr_nr: "1234567".to_string(),
                    lastname: "Mustermann".to_string(),
                    firstname: "Max".to_string(),
                    email: "max.mustermann@study.thws.de".to_string(),
                },
            ],
        };


        let app = test_helpers::init_api_test(routes::init_routes).await;
        let req = TestRequest::get().uri("/api/person").to_request();
        let resp: MainJson<Person> = test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp, expected_result, "Failed to get persons");
    }


    #[actix_web::test]
    async fn search_persons() {
        let expected_result = MainJson {
            page_number: 1,  
            pages_total: 1,  
            items_on_page: 1,
            items_total: 1,  
            data: vec![
                Person {
                    matr_nr: "1234567".to_string(),
                    lastname: "Mustermann".to_string(),
                    firstname: "Max".to_string(),
                    email: "max.mustermann@study.thws.de".to_string(),
                },
            ],
        };

        let app = test_helpers::init_api_test(routes::init_routes).await;
        let req = TestRequest::get().uri("/api/person?search=Max").to_request();
        let resp: MainJson<Person> = test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp, expected_result, "Failed to get searched persons");
    }


    #[actix_web::test]
    async fn insert_person_ok() {
        let person_to_insert = Person {
            matr_nr: "987654".to_string(),
            lastname: "Testnachname".to_string(),
            firstname: "Testvorname".to_string(),
            email: "testvorname.testnachname@study.thws.de".to_string(),
        };

        let app = test_helpers::init_api_test(routes::init_routes).await;
        let req = TestRequest::post().uri("/api/person").set_json(&person_to_insert).to_request();
        let resp: OperationResult = test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp.success, true, "Failed to insert person: {}", &resp.message);


        let url = format!("/api/person?id={}", person_to_insert.matr_nr);
        let req2 = TestRequest::get().uri(&url).to_request();
        let resp: MainJson<Person> = test::call_and_read_body_json(&app, req2).await;

        assert_eq!(resp.data.first(), Some(&person_to_insert), "Failed find person after inserting");
    }


    #[actix_web::test]
    async fn insert_person_fail() {
        let person_invalid_email = Person {
            matr_nr: "987654".to_string(),
            lastname: "Testnachname".to_string(),
            firstname: "Testvorname".to_string(),
            email: "ABCD".to_string(),
        };

        let person_empty_fields = Person {
            matr_nr: "".to_string(),
            lastname: "".to_string(),
            firstname: "".to_string(),
            email: "".to_string(),
        };

        let person_id_existing = Person {
            matr_nr: "1234567".to_string(),
            lastname: "Testnachname".to_string(),
            firstname: "Testvorname".to_string(),
            email: "testvorname.testnachname@study.thws.de".to_string(),
        };

        let app = test_helpers::init_api_test(routes::init_routes).await;
        let req = TestRequest::post().uri("/api/person").set_json(&person_invalid_email).to_request();
        let resp: OperationResult = test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp.success, false, "Error expected");
        assert_eq!(resp.message, "incorrect email".to_string(), "Wrong error message: {}", &resp.message);

        let req = TestRequest::post().uri("/api/person").set_json(&person_empty_fields).to_request();
        let resp: OperationResult = test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp.success, false, "Error expected");
        assert_eq!(resp.message, "Empty fields: matrNr, lastname, firstname, email".to_string(), "Wrong error message: {}", &resp.message);

        let req = TestRequest::post().uri("/api/person").set_json(&person_id_existing).to_request();
        let resp: OperationResult = test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp.success, false, "Error expected");
        assert_eq!(resp.message, "ID bereits vorhanden".to_string(), "Wrong error message: {}", &resp.message);
    }

    #[actix_web::test]
    async fn update_person_ok() {
        let old_person_id = "1234567";
        let person_to_update = Person {
            matr_nr: "987654".to_string(),
            lastname: "Testnachname".to_string(),
            firstname: "Testvorname".to_string(),
            email: "testvorname.testnachname@study.thws.de".to_string(),
        };

        let app = test_helpers::init_api_test(routes::init_routes).await;
        let url = format!("/api/person/{}", &old_person_id);
        let req = TestRequest::put().uri(&url).set_json(&person_to_update).to_request();
        let resp: OperationResult = test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp.success, true, "Failed to update person: {}", &resp.message);


        let url = format!("/api/person?id={}", person_to_update.matr_nr);
        let req2 = TestRequest::get().uri(&url).to_request();
        let resp: MainJson<Person> = test::call_and_read_body_json(&app, req2).await;

        assert_eq!(resp.data.first(), Some(&person_to_update), "Failed find person after updating");

        let url = format!("/api/person?id={}", &old_person_id);
        let req3= TestRequest::get().uri(&url).to_request();
        let resp: MainJson<Person> = test::call_and_read_body_json(&app, req3).await;

        assert!(resp.data.is_empty(), "Still found old person after updating");
    }

    #[actix_web::test]
    async fn delete_person_ok() {
        let old_person_id = "1234567";

        let app = test_helpers::init_api_test(routes::init_routes).await;
        let url = format!("/api/person/{}", &old_person_id);
        let req = TestRequest::delete().uri(&url).to_request();
        let resp: OperationResult = test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp.success, true, "Failed to delete person: {}", &resp.message);

        let url = format!("/api/person?id={}", &old_person_id);
        let req3= TestRequest::get().uri(&url).to_request();
        let resp: MainJson<Person> = test::call_and_read_body_json(&app, req3).await;

        assert!(resp.data.is_empty(), "Still found old person after deleting");
    }

    #[actix_web::test]
    async fn anonymize_person() {
        let expected_result_start = AnonymizePerson {
            person_available: true,
            person: Some(
                Person {
                    matr_nr: "ABC1234".to_string(),
                    lastname: "Musterfrau".to_string(),
                    firstname: "Erika".to_string(),
                    email: "erika.musterfrau@study.thws.de".to_string(),
                },
            ),
            days: Some(100),
        };

        let expected_result_exclude_from_anonymisation = OperationResult {
            success: true,
            message: "OK".to_string(),
        };

        let expected_result_end = AnonymizePerson {
            person_available: false,
            person: None,
            days: None,
        };

        let app = test_helpers::init_api_test(routes::init_routes).await;
        let req = TestRequest::get().uri("/api/person_to_anonymize").to_request();
        let resp: AnonymizePerson = test::call_and_read_body_json(&app, req).await;

        assert_eq!(&resp, &expected_result_start, "Person to anonymize wrong.");

        
        let url = format!("/api/person/{}/exclude_from_anonymisation", &resp.person.unwrap().matr_nr);
        println!("{}", &url);
        let req2= TestRequest::delete().uri(&url).to_request();
        let resp: OperationResult = test::call_and_read_body_json(&app, req2).await;
        
        assert_eq!(&resp, &expected_result_exclude_from_anonymisation, "Failed to exclude person from anonymisation.");
        
        
        let req3 = TestRequest::get().uri("/api/person_to_anonymize").to_request();
        let resp: AnonymizePerson = test::call_and_read_body_json(&app, req3).await;
        assert_eq!(&resp, &expected_result_end, "Still found person to anonymize after excluding.");
    }
}
