use crate::{
    database::FactureDatabase,
    models::{Customer, Invoice, Item},
    ui::{self, Tableable},
};
use anyhow::Result;
use chrono::Local;

pub fn list(database: &FactureDatabase) -> Result<()> {
    let invoices: Vec<Invoice> = database.read_all()?;
    let header = Vec::<Invoice>::header();
    let rows = invoices.rows();
    let table = ui::table(header, rows);
    println!("{table}");
    Ok(())
}

pub fn add(database: &FactureDatabase) -> Result<()> {
    let customers: Vec<Customer> = database.read_all()?;
    let mut customer = ui::prompt_select("Choose a customer to add the invoice", customers)?;
    let mut items: Vec<Item> = Default::default();
    let mut abort = false;

    while !abort {
        let description = ui::prompt_text("Description:")?;
        let price = ui::prompt_text("Price:")?;
        let quantity = ui::prompt_skipable_text("Quantity:")?;
        let quantity: Option<u32> = quantity.map(|quantity| quantity.parse().unwrap_or_default());
        let item = Item::builder()
            .description(description)
            .price(price.parse()?)
            .maybe_quantity(quantity)
            .build();
        items.push(item);
        abort = !ui::promt_confirm("Do you want to add another item")?;
    }

    let date = Local::now().date_naive();
    let invoice = Invoice::builder()
        .customer(&customer.uuid)
        .date(date)
        .items(items)
        .build();

    customer.invoices.push(invoice.uuid.clone());
    database.update(&customer.uuid, &customer)?;
    database.insert(&invoice)?;
    println!("Invoice created");

    Ok(())
}

pub fn remove(database: &FactureDatabase) -> Result<()> {
    let invoices: Vec<Invoice> = database.read_all()?;
    let invoice = ui::prompt_select("Choose a invoice to delete", invoices)?;

    database.remove(&invoice)?;
    println!("Invoice removed");

    Ok(())
}

pub fn edit(database: &FactureDatabase) -> Result<()> {
    let invoices: Vec<Invoice> = database.read_all()?;
    let invoice = ui::prompt_select("Choose an invoice to edit", invoices)?;
    let invoice_as_yaml = serde_yaml::to_string(&invoice)?;
    let invoice_as_yaml_edited =
        ui::prompt_editor("Open editor to edit invoice", &invoice_as_yaml, ".yaml")?;
    let invoice_edited: Invoice = serde_yaml::from_str(&invoice_as_yaml_edited)?;
    database.update(&invoice_edited.uuid, &invoice_edited)?;
    println!("Invoice edited");
    Ok(())
}

pub fn show(database: &FactureDatabase) -> Result<()> {
    let invoices: Vec<Invoice> = database.read_all()?;
    let invoice = ui::prompt_select("Choose an invoice to edit", invoices)?;
    let invoice_as_yaml = serde_yaml::to_string(&invoice)?;
    println!("{invoice_as_yaml}");
    Ok(())
}
