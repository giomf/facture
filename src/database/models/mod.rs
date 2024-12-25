pub mod business;
pub mod config;
pub mod customer;
pub mod invoice;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

const CONFIG_TABLE_NAME: &str = "config";

#[derive(Serialize, Default, Deserialize, Debug, Clone)]
pub struct Address {
    pub country: String,
    pub city: String,
    pub postal_code: String,
    pub street: String,
    pub number: String,
}

#[derive(Serialize, Default, Deserialize, Debug, Clone)]
pub struct Contact {
    pub name: String,
    pub surname: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

fn uuid_v7() -> String {
    Uuid::now_v7().to_string()
}
