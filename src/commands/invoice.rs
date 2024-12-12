use crate::{
    filesystem_database::FilesystemDatabase,
    models::{customer::Customer, invoice::Invoice, YamlAble},
    ui::prompt,
};
use anyhow::Result;

use super::{ListAble, CRUD};

impl YamlAble for Invoice {}
impl ListAble for Invoice {}

impl CRUD for Invoice {
    fn create(database: FilesystemDatabase, invoice: &Self, key: &str, name: &str) -> Result<()> {
        let customers: Vec<Customer> = database.read_all()?;
        let mut customer =
            prompt::select(&format!("Choose an customer to add an invoice"), customers)?;
        let mut invoice = invoice.clone();
        invoice.customer = customer.uuid.clone();
        let invoice_yaml = invoice.to_yaml()?;
        let invoice_yaml = prompt::editor(
            &format!("Open editor to edit {name}"),
            &invoice_yaml,
            ".yaml",
        )?;
        let invoice = Invoice::from_yaml(&invoice_yaml)?;
        database.create(key, invoice.clone())?;
        let old_customer = customer.clone();
        customer.add_invoice(&invoice.uuid);
        database.update(&old_customer.uuid, customer)?;
        println!("\n{invoice_yaml}");
        Ok(())
    }

    fn remove(database: FilesystemDatabase, key: &str) -> Result<()> {
        let invoice = database.read::<Invoice>(key)?;
        let mut customer = database.read::<Customer>(&invoice.customer)?;
        customer.remove_invoice(&invoice.uuid);
        database.delete::<Invoice>(&invoice.uuid)?;
        database.update(&customer.uuid, customer.clone())?;
        println!("Invoice removed");
        Ok(())
    }
}
