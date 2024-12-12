use super::{ListAble, CRUD};
use crate::{
    filesystem_database::FilesystemDatabase,
    models::{customer::Customer, invoice::Invoice, YamlAble},
    ui::prompt,
};
use anyhow::Result;

impl YamlAble for Customer {}
impl ListAble for Customer {}

impl CRUD for Customer {
    fn remove(database: FilesystemDatabase, key: &str) -> Result<()> {
        let result = prompt::confirm("This will also delete all invoices")?;
        if !result {
            println!("Aborted!");
            return Ok(());
        }
        let customer: Customer = database.read::<Customer>(key)?;
        database.delete::<Customer>(key)?;
        for invoice in customer.invoices {
            database.delete::<Invoice>(&invoice)?;
        }
        println!("Customer removed");

        Ok(())
    }
}
