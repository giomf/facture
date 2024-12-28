use crate::database::Model;

use super::{Address, Contact, CONFIG_TABLE_NAME};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Business {
    pub name: String,
    pub vat_id: String,
    pub vat: f32,
    pub contact: Contact,
    pub address: Address,
    pub payment: Payment,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Payment {
    pub bank: String,
    pub iban: String,
}
impl Model for Business {
    fn table() -> String {
        CONFIG_TABLE_NAME.to_owned()
    }
}
