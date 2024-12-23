use super::uuid_v7;
use crate::filesystem_database::Model;
use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};

const TABLE_NAME: &str = "invoices";

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct Invoice {
    pub uuid: String,
    pub id: String,
    pub date: NaiveDate,
    pub customer: String,
    pub items: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct Item {
    pub description: String,
    pub price: u32,
    pub quantity: Option<u32>,
}

impl Invoice {
    pub fn new_with_uuid(id: usize) -> Self {
        Self {
            uuid: uuid_v7(),
            id: format!("R{:05}", id),
            date: Local::now().date_naive(),
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
