use crate::{
    database::FactureDatabase,
    models::{Address, Contact, Customer},
    ui::{self, Tableable},
};
use anyhow::Result;

pub fn list(database: &FactureDatabase) -> Result<()> {
    let customers: Vec<Customer> = database.read_all()?;
    let header = Vec::<Customer>::header();
    let rows = customers.rows();
    let table = ui::table(header, rows);
    println!("{table}");
    Ok(())
}

pub fn add(database: &FactureDatabase) -> Result<()> {
    let organisation = ui::prompt_text("Organisation:")?;
    let name = ui::prompt_text("Name:")?;
    let surname = ui::prompt_text("Surname:")?;
    let email = ui::prompt_skipable_text("Email:")?;
    let phone = ui::prompt_skipable_text("Phone:")?;

    let country = ui::prompt_text("Country:")?;
    let city = ui::prompt_text("City:")?;
    let postal_code = ui::prompt_text("Postal Code:")?;
    let street = ui::prompt_text("Street:")?;
    let number = ui::prompt_text("House number:")?;

    let contact = Contact::builder()
        .name(name)
        .surname(surname)
        .maybe_email(email)
        .maybe_phone(phone)
        .build();

    let address = Address::builder()
        .country(country)
        .city(city)
        .postal_code(postal_code)
        .street(street)
        .number(number)
        .build();

    let customer = Customer::builder()
        .organisation(organisation)
        .contact(contact)
        .address(address)
        .build();

    database.insert(&customer)?;
    Ok(())
}

pub fn remove(database: &FactureDatabase) -> Result<()> {
    let customers: Vec<Customer> = database.read_all()?;
    let customer = ui::prompt_select("Choose a customer to delete", customers)?;
    let result = ui::promt_confirm("This will also delete all invoices")?;

    if !result {
        println!("Aborted!");
        return Ok(());
    }

    database.remove(&customer)?;
    println!("Customer removed");

    Ok(())
}

pub fn edit(database: &FactureDatabase) -> Result<()> {
    let customers: Vec<Customer> = database.read_all()?;
    let customer = ui::prompt_select("Choose a customer to edit", customers)?;
    let customer_as_yaml = serde_yaml::to_string(&customer)?;
    let customer_as_yaml_edited =
        ui::prompt_editor("Open editor to edit customer", &customer_as_yaml, ".yaml")?;
    let customer_edited: Customer = serde_yaml::from_str(&customer_as_yaml_edited)?;
    database.update(&customer_edited.uuid, &customer_edited)?;
    println!("Customer edited");
    Ok(())
}
pub fn show(database: &FactureDatabase) -> Result<()> {
    let customers: Vec<Customer> = database.read_all()?;
    let customer = ui::prompt_select("Choose an invoice to edit", customers)?;
    let customer_as_yaml = serde_yaml::to_string(&customer)?;
    println!("{customer_as_yaml}");
    Ok(())
}
