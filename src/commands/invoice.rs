use super::{render_id_template, ListAble, CRUD};
use crate::{
    cli::InvoiceCommand,
    commands::edit_object_in_temp_file,
    database::{
        models::{Business, Config, Customer, Invoice, BUSINESS_PRIMARY_KEY, CONFIG_PRIMARY_KEY},
        FactureDatabase, YamlAble,
    },
    template::{template, Template},
    ui::prompt,
};
use anyhow::Result;

impl YamlAble for Invoice {}
impl ListAble for Invoice {}

impl CRUD for Invoice {
    fn create(database: &FactureDatabase, invoice: &Self) -> Result<()> {
        let customers: Vec<Customer> = database.read_all()?;
        let mut customer =
            prompt::select(&format!("Choose an customer to add an invoice"), customers)?;
        let mut invoice = invoice.clone();
        invoice.customer = customer.uuid.clone();
        let invoice = edit_object_in_temp_file(&invoice)?;
        database.create(invoice.clone())?;
        let old_customer = customer.clone();
        customer.add_invoice(&invoice.uuid);
        database.update(&old_customer.uuid, customer)?;
        let invoice_yaml = invoice.to_yaml()?;
        println!("\n{invoice_yaml}");
        Ok(())
    }

    fn remove(database: &FactureDatabase, key: &str) -> Result<()> {
        let invoice = database.read::<Invoice>(key)?;
        let mut customer = database.read::<Customer>(&invoice.customer)?;
        customer.remove_invoice(&invoice.uuid);
        database.delete::<Invoice>(&invoice.uuid)?;
        database.update(&customer.uuid, customer.clone())?;
        println!("Invoice removed");
        Ok(())
    }
}

pub fn handle_invoice_command(command: &InvoiceCommand, database: FactureDatabase) -> Result<()> {
    let name = "invoice";
    match command {
        InvoiceCommand::List => Invoice::list(database)?,
        InvoiceCommand::Add => {
            let mut config = database.read::<Config>(CONFIG_PRIMARY_KEY)?;
            let invoice_id = render_id_template(
                &config.invoice_template,
                &format!("{:03}", config.invoice_counter),
            )?;
            let invoice = Invoice::new_with_uuid(invoice_id);
            Invoice::create(&database, &invoice)?;
            config.invoice_counter += 1;
            database.update(CONFIG_PRIMARY_KEY, config)?;
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
            let business: Business = database.read(BUSINESS_PRIMARY_KEY)?;
            let invoices: Vec<Invoice> = database.read_all()?;
            if invoices.is_empty() {
                println!("No invoices created yet");
                return Ok(());
            }
            let invoice = prompt::select(&format!("Select a {name} to render"), invoices)?;
            let customer: Customer = database.read(&invoice.customer)?;
            let template = Template::<template::Invoice>::new(business, customer, invoice)?;
            template.render()?;
        }
    }
    Ok(())
}
