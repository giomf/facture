pub mod templates;

use crate::database::{
    models::{business::Business, customer::Customer, invoice::Invoice},
    YamlAble,
};
use anyhow::Result;
use std::{fs, path::Path, process::Command};
use templates::TemplateInvoice;
use ureq;

const MAIN_TYP_CONTENT: &str = r#"
#import "./template.typ": *

#show: invoice.with(
  data: yaml("data.yaml"),
  styling: ( font: none ), // Explicitly use Typst's default font
)
"#;

const TEMPLATE_URL: &str =
    "https://raw.githubusercontent.com/ad-si/invoice-maker/refs/heads/main/invoice-maker.typ";
const OUTPUT_FOLDER: &str = "invoices";
const DATA_YAML_NAME: &str = "data.yaml";
const MAIN_TYP_NAME: &str = "main.typ";
const TEMPLATE_TYP_NAME: &str = "template.typ";
const COMPILE_COMMAND: &str = "typst";
const COMPILE_ARGUMENT: &str = "compile";

pub fn init() -> Result<()> {
    let output_folder = Path::new(OUTPUT_FOLDER);
    if output_folder.exists() {
        println!("Output folder already exists");
    } else {
        fs::create_dir_all(output_folder)?;
    }

    let main_typ = output_folder.join(MAIN_TYP_NAME);
    if main_typ.exists() {
        println!("Template entrypoint already exists");
    } else {
        fs::write(main_typ, MAIN_TYP_CONTENT)?;
    }
    let template_path = output_folder.join(TEMPLATE_TYP_NAME);
    if template_path.exists() {
        println!("Template already exists");
    } else {
        download_template(TEMPLATE_URL, output_folder)?;
    }
    Ok(())
}

fn download_template(url: &str, path: &Path) -> Result<()> {
    let content: String = ureq::get(url).call()?.into_string()?;
    let path = path.join(TEMPLATE_TYP_NAME);
    fs::write(path, content)?;
    Ok(())
}

fn compile(output_folder: &Path, invoice_name: &str) -> Result<()> {
    let entry_point = output_folder
        .join(MAIN_TYP_NAME)
        .to_string_lossy()
        .to_string();
    let output_pdf = output_folder
        .join(format!("{invoice_name}.pdf"))
        .to_string_lossy()
        .to_string();
    Command::new(COMPILE_COMMAND)
        .args([COMPILE_ARGUMENT, &entry_point, &output_pdf])
        .status()?;
    Ok(())
}

pub fn render(business: Business, customer: Customer, invoice: Invoice) -> Result<()> {
    let invoice_id = invoice.id.clone();
    let template = TemplateInvoice::new(business, customer, invoice);
    let template_yaml = template.to_yaml()?;
    let output_folder = Path::new(OUTPUT_FOLDER);
    fs::write(&output_folder.join(DATA_YAML_NAME), template_yaml)?;
    compile(&output_folder, &invoice_id)?;
    Ok(())
}
