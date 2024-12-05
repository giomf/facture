pub mod prompt;

use crate::models::customer::Customer;
use crate::models::invoice::Invoice;
use comfy_table::{presets::UTF8_FULL_CONDENSED, ContentArrangement, Table};
use std::fmt::Display;

pub trait Tableable {
    fn header() -> Vec<String>;
    fn rows(&self) -> Vec<Vec<String>>;
}

impl Tableable for Vec<Customer> {
    fn header() -> Vec<String> {
        vec![
            "Business".to_owned(),
            "Name".to_owned(),
            "Surname".to_owned(),
            "Email".to_owned(),
            "Phone".to_owned(),
            "Invoices".to_owned(),
        ]
    }

    fn rows(&self) -> Vec<Vec<String>> {
        self.into_iter()
            .map(|customer| {
                let customer = customer.clone();
                vec![
                    customer.organisation,
                    customer.contact.name,
                    customer.contact.surname,
                    customer.contact.email.unwrap_or_else(|| "n/a".to_owned()),
                    customer.contact.phone.unwrap_or_else(|| "n/a".to_owned()),
                    customer.invoices.len().to_string(),
                ]
            })
            .collect()
    }
}

impl Tableable for Vec<Invoice> {
    fn header() -> Vec<String> {
        vec!["Customer".to_owned(), "Date".to_owned()]
    }

    fn rows(&self) -> Vec<Vec<String>> {
        self.into_iter()
            .map(|invoice| vec![invoice.customer.clone(), invoice.date.clone().to_string()])
            .collect()
    }
}

impl Display for Customer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{} - {} {}",
            self.organisation, self.contact.name, self.contact.surname
        ))
    }
}

impl Display for Invoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self.uuid))
    }
}

pub fn table(header: Vec<String>, rows: Vec<Vec<String>>) -> String {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL_CONDENSED)
        .set_header(header)
        .set_content_arrangement(ContentArrangement::Dynamic);

    for row in rows {
        table.add_row(row);
    }

    table.to_string()
}
