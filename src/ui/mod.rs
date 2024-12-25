pub mod prompt;

use comfy_table::{presets::UTF8_FULL_CONDENSED, ContentArrangement, Table};
use std::fmt::Display;

use crate::database::models::{customer::Customer, invoice::Invoice};

pub trait TableAble {
    fn header() -> Vec<String>;
    fn row(self) -> Vec<String>;
}

impl TableAble for Customer {
    fn header() -> Vec<String> {
        vec![
            "ID".to_owned(),
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
            self.id,
            self.organization,
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
        vec!["ID".to_owned(), "Customer".to_owned(), "Date".to_owned()]
    }

    fn row(self) -> Vec<String> {
        vec![self.id, self.customer, self.issuing_date.to_string()]
    }
}

impl Display for Customer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{} - {} {}",
            self.organization, self.contact.name, self.contact.surname
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
