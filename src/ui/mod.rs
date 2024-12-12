pub mod prompt;

use crate::models::customer::Customer;
use crate::models::invoice::Invoice;
use comfy_table::{presets::UTF8_FULL_CONDENSED, ContentArrangement, Table};
use std::fmt::Display;

pub trait TableAble {
    fn header() -> Vec<String>;
    fn row(self) -> Vec<String>;
}

impl TableAble for Customer {
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

    fn row(self) -> Vec<String> {
        vec![
            self.organisation,
            self.contact.name,
            self.contact.surname,
            self.contact.email.unwrap_or_else(|| "n/a".to_owned()),
            self.contact.phone.unwrap_or_else(|| "n/a".to_owned()),
            self.invoices.len().to_string(),
        ]
    }
}

impl TableAble for Invoice {
    fn header() -> Vec<String> {
        vec!["Customer".to_owned(), "Date".to_owned()]
    }

    fn row(self) -> Vec<String> {
        vec![self.customer, self.date.to_string()]
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
