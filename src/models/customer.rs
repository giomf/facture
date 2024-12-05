use bon::Builder;
use native_db::*;
use native_model::{native_model, Model};
use serde::{Deserialize, Serialize};

use super::{uuid, YamlAble};

#[derive(Serialize, Deserialize, Debug, Builder, Clone, PartialEq, Eq)]
#[native_model(id = 1, version = 1)]
#[native_db]
#[builder(on(String, into))]
pub struct Customer {
    #[builder(default = uuid())]
    #[primary_key]
    pub uuid: String,
    pub organisation: String,
    pub contact: Contact,
    pub address: Address,
    #[builder(skip)]
    pub invoices: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Builder, Clone, PartialEq, Eq)]
pub struct Address {
    pub country: String,
    pub city: String,
    pub postal_code: String,
    pub street: String,
    pub number: String,
}

#[derive(Serialize, Deserialize, Debug, Builder, Clone, PartialEq, Eq)]
pub struct Contact {
    pub name: String,
    pub surname: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

impl YamlAble for Customer {}

impl Customer {
    pub fn add_invoice(&mut self, invoice_id: &str) {
        self.invoices.push(invoice_id.to_owned());
    }
    pub fn remove_invoice(&mut self, invoice_id: &str) {
        let new_invoices: Vec<_> = self
            .invoices
            .clone()
            .into_iter()
            .filter(|current_invoice_id| *current_invoice_id == invoice_id)
            .collect();
        self.invoices = new_invoices;
    }
}
