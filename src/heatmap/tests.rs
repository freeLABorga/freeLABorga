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

    use crate::models::{MainJson};
    use crate::test_helpers;
    use super::super::{routes, Gegenstand};

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
                    price: String::from("19,95 €"),
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
                    price: String::from("423,50 €"),
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
                    price: String::from("423,50 €"),
                    buydate: String::from("2020-10-04"),
                    inventoried: Some(true),
                    available: Some(false),
                    place: String::from("Labor"),
                    categories: vec![String::from("Linux"), String::from("Schwarz"), String::from("Smartphone")],
                },
                
            ],
        };


        let app = test_helpers::init_api_test(routes::init_routes).await;
        let req = TestRequest::get().uri("/api/heatmap").to_request();
        let resp: MainJson<Gegenstand> = test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp, expected_result, "Failed to get lab items");
    }
}
