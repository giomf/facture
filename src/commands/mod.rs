pub mod business;
pub mod customer;
pub mod invoice;

use crate::{
    cli::{BusinessCommand, ConfigCommand, CustomerCommand, InvoiceCommand},
    database::{
        models::{business::Business, config::Config, customer::Customer, invoice::Invoice},
        FilesystemDatabase, Model, YamlAble,
    },
    renderer::{self},
    ui::{self, prompt, TableAble},
};
use anyhow::Result;
use std::{env, fs, process::Command};
use tempfile::Builder;

const BUSINESS_KEY: &str = "business";
const CONFIG_KEY: &str = "config";

pub trait ListAble: Model + TableAble {
    fn list(database: FilesystemDatabase) -> Result<()> {
        let objects: Vec<Self> = database.read_all()?;
        if objects.is_empty() {
            println!("Nothing created yet.");
            return Ok(());
        }
        let header = Self::header();
        let rows = objects.into_iter().map(|object| object.row()).collect();
        let table = ui::table(header, rows);
        println!("{table}");
        Ok(())
    }
}

pub trait CRUD: Clone + Model {
    fn create(database: &FilesystemDatabase, object: &Self, key: &str) -> Result<()> {
        let new_object = edit_object_in_temp_file(object)?;
        database.create(key, new_object.clone())?;
        let new_object_yaml = new_object.to_yaml()?;
        println!("\n{new_object_yaml}");
        Ok(())
    }

    fn show(object: &Self) -> Result<()> {
        let object_yaml = object.to_yaml()?;
        println!("{object_yaml}");
        Ok(())
    }

    fn edit(database: &FilesystemDatabase, object: &Self, key: &str) -> Result<()> {
        let new_object = edit_object_in_temp_file(object)?;
        database.update(key, new_object.clone())?;
        let new_object_yaml = new_object.to_yaml()?;
        println!("\n{new_object_yaml}");
        Ok(())
    }

    fn remove(database: &FilesystemDatabase, key: &str) -> Result<()> {
        database.delete::<Self>(key)?;
        Ok(())
    }
}

fn edit_object_in_temp_file<T: YamlAble>(object: &T) -> Result<T> {
    let object_yaml = object.to_yaml()?;
    let temp_file = Builder::new()
        .prefix("facture_")
        .suffix(".yaml")
        .tempfile()?;
    fs::write(temp_file.path(), object_yaml)?;
    let editor = env::var("EDITOR")?;
    Command::new(editor).arg(temp_file.path()).status()?;
    let object_yaml = fs::read_to_string(temp_file.path())?;
    let new_object = T::from_yaml(&object_yaml)?;
    Ok(new_object)
}

pub fn handle_customer_command(
    command: &CustomerCommand,
    database: FilesystemDatabase,
) -> Result<()> {
    let name = "customer";

    match command {
        CustomerCommand::List => Customer::list(database)?,
        CustomerCommand::Add => {
            let mut config = database.read::<Config>(CONFIG_KEY)?;
            let customer =
                Customer::new_with_uuid(&config.customer_prefix, config.customer_counter);
            Customer::create(&database, &customer, &customer.uuid)?;
            config.customer_counter += 1;
            database.update(CONFIG_KEY, config)?;
        }
        CustomerCommand::Remove => {
            let customers: Vec<Customer> = database.read_all()?;
            if customers.is_empty() {
                println!("No customers created yet");
                return Ok(());
            }
            let customer = prompt::select(&format!("Select a {name} to remove"), customers)?;
            Customer::remove(&database, &customer.uuid)?;
        }
        CustomerCommand::Edit => {
            let customers: Vec<Customer> = database.read_all()?;
            if customers.is_empty() {
                println!("No customers created yet");
                return Ok(());
            }
            let customer = prompt::select(&format!("Select a {name} to edit"), customers)?;
            Customer::edit(&database, &customer, &customer.uuid)?;
        }
        CustomerCommand::Show => {
            let customers: Vec<Customer> = database.read_all()?;
            if customers.is_empty() {
                println!("No customers created yet");
                return Ok(());
            }
            let customer = prompt::select(&format!("Select a {name} to show"), customers)?;
            Customer::show(&customer)?;
        }
    }
    Ok(())
}

pub fn handle_invoice_command(
    command: &InvoiceCommand,
    database: FilesystemDatabase,
) -> Result<()> {
    let name = "invoice";
    match command {
        InvoiceCommand::List => Invoice::list(database)?,
        InvoiceCommand::Add => {
            let mut config = database.read::<Config>(CONFIG_KEY)?;
            let invoice = Invoice::new_with_uuid(&config.invoice_prefix, config.invoice_counter);
            Invoice::create(&database, &invoice, &invoice.uuid)?;
            config.invoice_counter += 1;
            database.update(CONFIG_KEY, config)?;
        }
        InvoiceCommand::Remove => {
            let invoices: Vec<Invoice> = database.read_all()?;
            if invoices.is_empty() {
                println!("No invoices created yet");
                return Ok(());
            }
            let invoice = prompt::select(&format!("Select a {name} to remove"), invoices)?;
            Invoice::remove(&database, &invoice.uuid)?;
        }
        InvoiceCommand::Edit => {
            let invoices: Vec<Invoice> = database.read_all()?;
            if invoices.is_empty() {
                println!("No invoices created yet");
                return Ok(());
            }

            let invoice = prompt::select(&format!("Select a {name} to edit"), invoices)?;
            Invoice::edit(&database, &invoice, &invoice.uuid)?;
        }
        InvoiceCommand::Show => {
            let invoices: Vec<Invoice> = database.read_all()?;
            if invoices.is_empty() {
                println!("No invoices created yet");
                return Ok(());
            }
            let invoice = prompt::select(&format!("Select a {name} to show"), invoices)?;
            Invoice::show(&invoice)?;
        }
        InvoiceCommand::Render => {
            let business: Business = database.read(BUSINESS_KEY)?;
            let invoices: Vec<Invoice> = database.read_all()?;
            if invoices.is_empty() {
                println!("No invoices created yet");
                return Ok(());
            }
            let invoice = prompt::select(&format!("Select a {name} to show"), invoices)?;
            let customer: Customer = database.read(&invoice.customer)?;
            renderer::render(business, customer, invoice)?;
        }
    }
    Ok(())
}

pub fn handle_business_command(
    command: &BusinessCommand,
    database: FilesystemDatabase,
) -> Result<()> {
    match command {
        BusinessCommand::Init => {
            // Init database models
            database.define::<Business>()?;
            database.define::<Customer>()?;
            database.define::<Invoice>()?;
            database.define::<Config>()?;

            // Init config
            if database.exists::<Config>(CONFIG_KEY)? {
                println!("Config already exists");
            } else {
                let config = Config::default();
                Config::create(&database, &config, CONFIG_KEY)?;
            };

            // Init business
            if database.exists::<Business>(BUSINESS_KEY)? {
                println!("Business already exists");
            } else {
                let business = Business::default();
                Business::create(&database, &business, BUSINESS_KEY)?;
            }

            // Init renderer
            renderer::init()?;
        }
        BusinessCommand::Edit => {
            let business = database.read::<Business>(BUSINESS_KEY)?;
            Business::edit(&database, &business, BUSINESS_KEY)?;
        }
        BusinessCommand::Show => {
            let business = database.read::<Business>(BUSINESS_KEY)?;
            Business::show(&business)?;
        }
    }
    Ok(())
}

pub fn handle_config_command(command: &ConfigCommand, database: FilesystemDatabase) -> Result<()> {
    match command {
        ConfigCommand::Edit => {
            let config = database.read::<Config>(CONFIG_KEY)?;
            Config::edit(&database, &config, CONFIG_KEY)?;
        }
        ConfigCommand::Show => {
            let config = database.read::<Config>(CONFIG_KEY)?;
            Config::show(&config)?;
        }
    }

    Ok(())
}
