use super::{uuid_v7, Address, Contact};
use native_db::{native_db, ToKey};
use native_model::{native_model, Model};
use serde::{Deserialize, Serialize};

pub type Customer = v1::Customer;

pub mod v1 {
    use super::*;

    #[native_db]
    #[native_model(id = 1, version = 1)]
    #[derive(Serialize, Deserialize, Debug, Clone, Default)]
    pub struct Customer {
        #[primary_key]
        pub uuid: String,
        pub id: String,
        pub organization: String,
        pub vat_id: String,
        pub contact: Contact,
        pub address: Address,
        pub invoices: Vec<String>,
    }
}

impl Customer {
    pub fn new_with_uuid(prefix: &str, id: usize) -> Self {
        Self {
            uuid: uuid_v7(),
            id: format!("{prefix}{:05}", id),
            ..Default::default()
        }
    }

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
