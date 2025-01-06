mod business;
mod config;
mod customer;
mod invoice;

pub use business::{Business, PRIMARY_KEY as BUSINESS_PRIMARY_KEY};
pub use config::{Config, PRIMARY_KEY as CONFIG_PRIMARY_KEY};
pub use customer::Customer;
pub use invoice::Invoice;
pub use invoice::Item;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod v1 {
    pub use super::business::v1::*;
    pub use super::config::v1::*;
    pub use super::customer::v1::*;
    pub use super::invoice::v1::*;
}

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
