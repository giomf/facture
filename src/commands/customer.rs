use super::{ListAble, CRUD};
use crate::{
    cli::CustomerCommand,
    database::{
        models::{Config, Customer, Invoice, CONFIG_PRIMARY_KEY},
        FactureDatabase, YamlAble,
    },
    ui::prompt,
};
use anyhow::Result;

impl YamlAble for Customer {}
impl ListAble for Customer {}

impl CRUD for Customer {
    fn remove(database: &FactureDatabase, key: &str) -> Result<()> {
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
pub fn handle_customer_command(command: &CustomerCommand, database: FactureDatabase) -> Result<()> {
    let name = "customer";

    match command {
        CustomerCommand::List => Customer::list(database)?,
        CustomerCommand::Add => {
            let mut config = database.read::<Config>(CONFIG_PRIMARY_KEY)?;
            let customer =
                Customer::new_with_uuid(&config.customer_prefix, config.customer_counter);
            Customer::create(&database, &customer)?;
            config.customer_counter += 1;
            database.update(CONFIG_PRIMARY_KEY, config)?;
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
