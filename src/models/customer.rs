use super::{uuid_v7, Address, Contact};
use crate::filesystem_database::Model;
use serde::{Deserialize, Serialize};

const TABLE_NAME: &str = "customers";

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Customer {
    pub uuid: String,
    pub id: String,
    pub organisation: String,
    pub contact: Contact,
    pub address: Address,
    pub invoices: Vec<String>,
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

impl Model for Customer {
    fn table() -> String {
        TABLE_NAME.to_owned()
    }
}
