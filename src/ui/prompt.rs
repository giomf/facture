use anyhow::Result;
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

pub fn text(message: &str) -> Result<String> {
    let validator = ValueRequiredValidator::new("A value is required for this field");
    let text = Text::new(message)
        .with_render_config(*RENDER_CONFIG)
        .with_validator(validator)
        .prompt()?;
    Ok(text)
}

pub fn skipable_text(message: &str) -> Result<Option<String>> {
    let validator =
        ValueRequiredValidator::new("A value is required for this field. Skip it with ESC");
    let text = Text::new(message)
        .with_render_config(*RENDER_CONFIG)
        .with_validator(validator)
        .with_help_message("This field is optional - Skip it with ESC")
        .prompt_skippable()?;
    Ok(text)
}

pub fn editor(message: &str, file_content: &str, file_extension: &str) -> Result<String> {
    let content = Editor::new(message)
        .with_predefined_text(file_content)
        .with_file_extension(file_extension)
        .prompt()?;
    Ok(content)
}

pub fn select<T: Display>(message: &str, options: Vec<T>) -> Result<T> {
    let answer = Select::new(message, options)
        .with_render_config(*RENDER_CONFIG)
        .prompt()?;

    Ok(answer)
}

pub fn confirm(message: &str) -> Result<bool> {
    let answer = Confirm::new(message)
        .with_default(false)
        .with_render_config(*RENDER_CONFIG)
        .prompt()?;
    Ok(answer)
}
