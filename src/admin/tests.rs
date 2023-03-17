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
    use actix_web::web::Bytes;

    use crate::models::OperationResult;
    use crate::test_helpers;
    use super::super::{routes, Admin};

    #[actix_web::test]
    async fn get_admin_texts() {
        let expected_result = Admin {
            imprint_text: "<p>Dies ist das Impressum</p>".to_string(),
            imprint_html: true,
            privacy_text: "Dies ist die\nDatenschutzerklärung<br>".to_string(),
            privacy_html: false,
            items_per_page: Some(20),
            days_until_anonymize: Some(100),
        };

        let app = test_helpers::init_api_test(routes::init_routes).await;
        let req = TestRequest::get().uri("/api/admin").to_request();
        let mut resp: Admin = test::call_and_read_body_json(&app, req).await;

        // Löschen von \r (Unterschied zwischen Windows und Linux)
        resp.imprint_text = resp.imprint_text.replace("\r", "");
        resp.privacy_text = resp.privacy_text.replace("\r", "");

        assert_eq!(resp, expected_result, "Failed to get admin data");
    }

    #[actix_web::test]
    async fn update_admin_texts_ok() {
        let updated_admin_texts = Admin {
            imprint_text: "<p>Dies ist das veränderte Impressum</p>".to_string(),
            imprint_html: false,
            privacy_text: "<p>Dies ist die veränderte Datenschutzerklärung</p>".to_string(),
            privacy_html: true,
            items_per_page: Some(50),
            days_until_anonymize: Some(80),
        };

        let app = test_helpers::init_api_test(routes::init_routes).await;
        let req = TestRequest::put().uri("/api/admin").set_json(&updated_admin_texts).to_request();
        let resp: OperationResult = test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp.success, true, "Failed to update admin texts: {}", &resp.message);


        let req2 = TestRequest::get().uri("/api/admin").to_request();
        let resp: Admin = test::call_and_read_body_json(&app, req2).await;

        assert_eq!(resp, updated_admin_texts, "Failed get right data after updating admin texts.");
    }

    #[actix_web::test]
    async fn get_imprint() {
        let expected_result = Bytes::from_static(b"<p>Dies ist das Impressum</p>");

        let app = test_helpers::init_api_test(routes::init_routes).await;
        let req = TestRequest::get().uri("/api/impressum").to_request();
        let resp = test::call_and_read_body(&app, req).await;

        assert_eq!(resp, expected_result, "Failed to get impressum data");
    }

    #[actix_web::test]
    async fn get_privacy() {
        let expected_result = Bytes::from_static(b"Dies ist die<br>Datenschutzerkl\xc3\xa4rung&lt;br&gt;");

        let app = test_helpers::init_api_test(routes::init_routes).await;
        let req = TestRequest::get().uri("/api/datenschutz").to_request();
        let resp = test::call_and_read_body(&app, req).await;

        assert_eq!(resp, expected_result, "Failed to get impressum data");
    }
}
