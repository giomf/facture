pub mod business;
pub mod customer;
pub mod invoice;

use crate::{
    cli::{BusinessCommand, ItemCommand},
    filesystem_database::{FilesystemDatabase, Model},
    models::{business::Business, customer::Customer, invoice::Invoice, YamlAble},
    ui::{self, prompt, TableAble},
};
use anyhow::Result;

const BUSINESS_KEY: &str = "business";

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

pub trait CRUD: Clone + Model + YamlAble {
    fn create(database: FilesystemDatabase, object: &Self, key: &str, name: &str) -> Result<()> {
        let object_yaml = object.to_yaml()?;
        let object_yaml = prompt::editor(
            &format!("Open editor to create {name}"),
            &object_yaml,
            ".yaml",
        )?;
        let object = Self::from_yaml(&object_yaml)?;
        database.create(key, object.clone())?;
        let object_yaml = object.to_yaml()?;
        println!("\n{object_yaml}");
        Ok(())
    }

    fn show(object: &Self) -> Result<()> {
        let object_yaml = object.to_yaml()?;
        println!("{object_yaml}");
        Ok(())
    }

    fn edit(database: FilesystemDatabase, object: &Self, key: &str, name: &str) -> Result<()> {
        let object_yaml = object.to_yaml()?;
        let new_object_yaml = prompt::editor(
            &format!("Open editor to edit {name}"),
            &object_yaml,
            ".yaml",
        )?;
        let new_object = Self::from_yaml(&new_object_yaml)?;
        database.update(key, new_object.clone())?;
        let new_object_yaml = new_object.to_yaml()?;
        println!("\n{new_object_yaml}");
        Ok(())
    }

    fn remove(database: FilesystemDatabase, key: &str) -> Result<()> {
        database.delete::<Self>(key)?;
        Ok(())
    }
}

pub fn handle_customer_command(command: &ItemCommand, database: FilesystemDatabase) -> Result<()> {
    let name = "customer";

    match command {
        ItemCommand::List => Customer::list(database)?,
        ItemCommand::Add => {
            let customer = Customer::new_with_uuid(0);
            Customer::create(database, &customer, &customer.uuid, name)?;
        }
        ItemCommand::Remove => {
            let customers: Vec<Customer> = database.read_all()?;
            let customer = prompt::select(&format!("Select a {name} to remove"), customers)?;
            Customer::remove(database, &customer.uuid)?;
        }
        ItemCommand::Edit => {
            let customers: Vec<Customer> = database.read_all()?;
            let customer = prompt::select(&format!("Select a {name} to edit"), customers)?;
            Customer::edit(database, &customer, &customer.uuid, name)?;
        }
        ItemCommand::Show => {
            let customers: Vec<Customer> = database.read_all()?;
            let customer = prompt::select(&format!("Select a {name} to show"), customers)?;
            Customer::show(&customer)?;
        }
    }
    Ok(())
}

pub fn handle_invoice_command(command: &ItemCommand, database: FilesystemDatabase) -> Result<()> {
    let name = "invoice";
    match command {
        ItemCommand::List => Invoice::list(database)?,
        ItemCommand::Add => {
            let invoice = Invoice::new_with_uuid(0);
            Invoice::create(database, &invoice, &invoice.uuid, name)?;
        }
        ItemCommand::Remove => {
            let invoices: Vec<Invoice> = database.read_all()?;
            let invoice = prompt::select(&format!("Select a {name} to remove"), invoices)?;
            Invoice::remove(database, &invoice.uuid)?;
        }
        ItemCommand::Edit => {
            let invoices: Vec<Invoice> = database.read_all()?;
            let invoice = prompt::select(&format!("Select a {name} to edit"), invoices)?;
            Invoice::edit(database, &invoice, &invoice.uuid, name)?;
        }
        ItemCommand::Show => {
            let invoices: Vec<Invoice> = database.read_all()?;
            let invoice = prompt::select(&format!("Select a {name} to show"), invoices)?;
            Invoice::show(&invoice)?;
        }
    }
    Ok(())
}

pub fn handle_business_command(
    command: &BusinessCommand,
    database: FilesystemDatabase,
) -> Result<()> {
    let name = "business";
    match command {
        BusinessCommand::Init => {
            let business = Business::default();
            Business::create(database, &business, BUSINESS_KEY, name)?;
        }
        BusinessCommand::Edit => {
            let business = database.read::<Business>(BUSINESS_KEY)?;
            Business::edit(database, &business, BUSINESS_KEY, name)?;
        }
        BusinessCommand::Show => {
            let business = database.read::<Business>(BUSINESS_KEY)?;
            Business::show(&business)?;
        }
    }
    Ok(())
}
