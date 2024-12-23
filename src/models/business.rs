use super::{Address, Contact, CONFIG_TABLE_NAME};
use crate::filesystem_database::Model;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Business {
    pub organisation: String,
    pub contact: Contact,
    pub address: Address,
    pub payment: Payment,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Payment {
    bank: String,
    iban: String,
}
impl Model for Business {
    fn table() -> String {
        CONFIG_TABLE_NAME.to_owned()
    }
}
