use super::{ListAble, CRUD};
use crate::{
    commands::edit_object_in_temp_file,
    database::{
        models::{customer::Customer, invoice::Invoice},
        FilesystemDatabase, YamlAble,
    },
    ui::prompt,
};
use anyhow::Result;

impl YamlAble for Invoice {}
impl ListAble for Invoice {}

impl CRUD for Invoice {
    fn create(database: &FilesystemDatabase, invoice: &Self, key: &str) -> Result<()> {
        let customers: Vec<Customer> = database.read_all()?;
        let mut customer =
            prompt::select(&format!("Choose an customer to add an invoice"), customers)?;
        let mut invoice = invoice.clone();
        invoice.customer = customer.uuid.clone();
        let invoice = edit_object_in_temp_file(&invoice)?;
        database.create(key, invoice.clone())?;
        let old_customer = customer.clone();
        customer.add_invoice(&invoice.uuid);
        database.update(&old_customer.uuid, customer)?;
        let invoice_yaml = invoice.to_yaml()?;
        println!("\n{invoice_yaml}");
        Ok(())
    }

    fn remove(database: &FilesystemDatabase, key: &str) -> Result<()> {
        let invoice = database.read::<Invoice>(key)?;
        let mut customer = database.read::<Customer>(&invoice.customer)?;
        customer.remove_invoice(&invoice.uuid);
        database.delete::<Invoice>(&invoice.uuid)?;
        database.update(&customer.uuid, customer.clone())?;
        println!("Invoice removed");
        Ok(())
    }
}
