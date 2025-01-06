mod typst_invoice;

use crate::database::{
    models::{Business, Customer, Invoice},
    YamlAble,
};
use anyhow::Result;
use std::{fs, path::Path, process::Command};
pub use typst_invoice::template;
use ureq;

const OUTPUT_FOLDER: &str = "invoices";
const DATA_YAML_NAME: &str = "data.yaml";
const TEMPLATE_MAIN_NAME: &str = "main.typ";
const TEMPLATE_LIB_NAME: &str = "template.typ";
const COMPILE_COMMAND: &str = "typst";
const COMPILE_ARGUMENT: &str = "compile";

pub struct Template<T: TemplateAble> {
    invoice_id: String,
    template: T,
}

impl<T: TemplateAble> Template<T> {
    pub fn new(business: Business, customer: Customer, invoice: Invoice) -> Result<Self> {
        Self::init()?;
        let template = Self {
            invoice_id: invoice.id.clone(),
            template: T::new(business, customer, invoice),
        };

        Ok(template)
    }

    fn init() -> Result<()> {
        let output_folder = Path::new(OUTPUT_FOLDER);
        if output_folder.exists() {
            println!("Output folder already exists");
        } else {
            fs::create_dir_all(output_folder)?;
        }

        let template_main = output_folder.join(TEMPLATE_MAIN_NAME);
        if template_main.exists() {
            println!("Template entrypoint already exists");
        } else {
            fs::write(template_main, T::main())?;
        }
        let template_lib = output_folder.join(TEMPLATE_LIB_NAME);
        if template_lib.exists() {
            println!("Template already exists");
        } else {
            Self::download_template(&T::url(), &template_lib)?;
        }
        Ok(())
    }

    fn download_template(url: &str, path: &Path) -> Result<()> {
        let content: String = ureq::get(url).call()?.into_string()?;
        fs::write(path, content)?;
        Ok(())
    }

    fn compile(output_folder: &Path, invoice_name: &str) -> Result<()> {
        let template_main = output_folder
            .join(TEMPLATE_MAIN_NAME)
            .to_string_lossy()
            .to_string();
        let output_pdf = output_folder
            .join(format!("{invoice_name}.pdf"))
            .to_string_lossy()
            .to_string();
        Command::new(COMPILE_COMMAND)
            .args([COMPILE_ARGUMENT, &template_main, &output_pdf])
            .status()?;
        Ok(())
    }

    pub fn render(&self) -> Result<()> {
        let template_yaml = self.template.to_yaml()?;
        let output_folder = Path::new(OUTPUT_FOLDER);
        fs::write(&output_folder.join(DATA_YAML_NAME), template_yaml)?;
        Self::compile(&output_folder, &self.invoice_id)?;
        Ok(())
    }
}

pub trait TemplateAble: Sized + YamlAble {
    fn new(business: Business, customer: Customer, invoice: Invoice) -> Self;
    fn main() -> String;
    fn url() -> String;
}
