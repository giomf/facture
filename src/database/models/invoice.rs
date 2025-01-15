use super::uuid_v7;
use chrono::{Local, NaiveDate};
use native_db::{native_db, ToKey};
use native_model::{native_model, Model};
use serde::{Deserialize, Serialize};

pub type Invoice = v1::Invoice;
pub type Item = v1::Item;

pub mod v1 {
    use super::*;

    #[native_db]
    #[native_model(id = 2, version = 1)]
    #[derive(Serialize, Deserialize, Debug, Default, Clone)]
    pub struct Invoice {
        #[primary_key]
        pub uuid: String,
        pub id: String,
        pub issuing_date: NaiveDate,
        pub delivery_date: NaiveDate,
        pub due_days: u32,
        pub customer: String,
        pub items: Vec<Item>,
    }
    #[derive(Serialize, Deserialize, Debug, Default, Clone)]
    pub struct Item {
        pub description: String,
        pub price: f32,
        pub quantity: Option<u32>,
    }
}

impl Invoice {
    pub fn new_with_uuid(id: String) -> Self {
        let date = Local::now().date_naive();
        Self {
            uuid: uuid_v7(),
            id,
            issuing_date: date,
            delivery_date: date,
            due_days: 30,
            items: vec![Item::default()],
            ..Default::default()
        }
    }
}
