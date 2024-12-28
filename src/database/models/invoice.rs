use crate::database::Model;

use super::uuid_v7;
use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};

const TABLE_NAME: &str = "invoices";

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Invoice {
    pub uuid: String,
    pub id: String,
    pub issuing_date: NaiveDate,
    pub delivery_date: NaiveDate,
    pub customer: String,
    pub items: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Item {
    pub description: String,
    pub price: f32,
    pub quantity: Option<u32>,
}

impl Invoice {
    pub fn new_with_uuid(prefix: &str, id: usize) -> Self {
        let date = Local::now().date_naive();
        Self {
            uuid: uuid_v7(),
            id: format!("{prefix}{:05}", id),
            issuing_date: date,
            delivery_date: date,
            items: vec![Item::default()],
            ..Default::default()
        }
    }
}

impl Model for Invoice {
    fn table() -> String {
        TABLE_NAME.to_owned()
    }
}
