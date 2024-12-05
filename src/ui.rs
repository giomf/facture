use crate::models::{Customer, Invoice};
use anyhow::Result;
use comfy_table::{presets::UTF8_FULL_CONDENSED, ContentArrangement, Table};
use inquire::{
    ui::{RenderConfig, StyleSheet},
    validator::ValueRequiredValidator,
    Confirm, Editor, Select, Text,
};
use std::{fmt::Display, sync::LazyLock};

static RENDER_CONFIG: LazyLock<RenderConfig> = LazyLock::new(|| {
    RenderConfig::default_colored()
        .with_help_message(StyleSheet::new().with_fg(inquire::ui::Color::Grey))
        .with_selected_option(Some(
            StyleSheet::new().with_fg(inquire::ui::Color::DarkYellow),
        ))
});

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

pub fn prompt_text(message: &str) -> Result<String> {
    let validator = ValueRequiredValidator::new("A value is required for this field");
    let text = Text::new(message)
        .with_render_config(*RENDER_CONFIG)
        .with_validator(validator)
        .prompt()?;
    Ok(text)
}

pub fn prompt_skipable_text(message: &str) -> Result<Option<String>> {
    let validator =
        ValueRequiredValidator::new("A value is required for this field. Skip it with ESC");
    let text = Text::new(message)
        .with_render_config(*RENDER_CONFIG)
        .with_validator(validator)
        .with_help_message("This field is optional - Skip it with ESC")
        .prompt_skippable()?;
    Ok(text)
}

pub fn prompt_editor(message: &str, file_content: &str, file_extension: &str) -> Result<String> {
    let content = Editor::new(message)
        .with_predefined_text(file_content)
        .with_file_extension(file_extension)
        .prompt()?;
    Ok(content)
}

pub fn prompt_select<T: Display>(message: &str, options: Vec<T>) -> Result<T> {
    let answer = Select::new(message, options)
        .with_render_config(*RENDER_CONFIG)
        .prompt()?;

    Ok(answer)
}

pub fn promt_confirm(message: &str) -> Result<bool> {
    let answer = Confirm::new(message)
        .with_default(false)
        .with_render_config(*RENDER_CONFIG)
        .prompt()?;
    Ok(answer)
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
