use super::{Address, Contact};

use native_db::{native_db, ToKey};
use native_model::{native_model, Model};
use serde::{Deserialize, Serialize};

pub const PRIMARY_KEY: &str = "BUSINESS";

pub type Business = v1::Business;

pub mod v1 {
    use super::*;

    #[native_db(primary_key(primary_key -> String))]
    #[native_model(id = 3, version = 1)]
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
}

impl Business {
    fn primary_key(&self) -> String {
        PRIMARY_KEY.to_owned()
    }
}
