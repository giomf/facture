use crate::{
    database::FactureDatabase,
    models::{
        customer::{Address, Contact, Customer},
        YamlAble,
    },
    ui::{self, prompt, Tableable},
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
    let organisation = prompt::text("Organisation:")?;
    let name = prompt::text("Name:")?;
    let surname = prompt::text("Surname:")?;
    let email = prompt::skipable_text("Email:")?;
    let phone = prompt::skipable_text("Phone:")?;

    let country = prompt::text("Country:")?;
    let city = prompt::text("City:")?;
    let postal_code = prompt::text("Postal Code:")?;
    let street = prompt::text("Street:")?;
    let number = prompt::text("House number:")?;

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
    let customer = prompt::select("Choose a customer to delete", customers)?;
    let result = prompt::confirm("This will also delete all invoices")?;

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
    let customer = prompt::select("Choose a customer to edit", customers)?;
    let customer_as_yaml = serde_yaml::to_string(&customer)?;
    let customer_as_yaml_edited =
        prompt::editor("Open editor to edit customer", &customer_as_yaml, ".yaml")?;
    let customer_edited: Customer = serde_yaml::from_str(&customer_as_yaml_edited)?;
    database.update(&customer_edited.uuid, &customer_edited)?;
    println!("Customer edited");
    Ok(())
}
pub fn show(database: &FactureDatabase) -> Result<()> {
    let customers: Vec<Customer> = database.read_all()?;
    let customer = prompt::select("Choose an invoice to edit", customers)?;
    let customer_as_yaml = customer.to_yaml()?;
    println!("{customer_as_yaml}");
    Ok(())
}
