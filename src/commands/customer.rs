use super::Command;
use crate::database::{
    create_connection,
    customer::{CustomerRepository, NewCustomer, UpdateCustomer},
    Repository, DATABASE_PATH,
};
use clap::{Args, Subcommand};
use std::path::Path;

#[derive(Args, Clone, Debug)]
pub struct CreateArgs {
    /// Customer name
    #[arg(long)]
    pub name: String,

    /// Customer surname
    #[arg(long)]
    pub surname: String,

    /// Customer email address
    #[arg(long)]
    pub email: Option<String>,

    /// Customer phone number
    #[arg(long)]
    pub phone: Option<String>,
}

#[derive(Args, Clone, Debug)]
pub struct ShowArgs {
    /// Customer ID
    pub id: i32,
}

#[derive(Args, Clone, Debug)]
pub struct DeleteArgs {
    /// Customer ID
    pub id: i32,
}

#[derive(Args, Clone, Debug)]
pub struct EditArgs {
    /// Customer ID
    pub id: i32,

    /// Customer name
    #[arg(long)]
    pub name: Option<String>,

    /// Customer surname
    #[arg(long)]
    pub surname: Option<String>,

    /// Customer email address
    #[arg(long)]
    pub email: Option<String>,

    /// Customer phone number
    #[arg(long)]
    pub phone: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum CustomerCommand {
    /// Create a new customer
    Create(CreateArgs),

    /// Show a customer
    Show(ShowArgs),

    /// Delete a customer
    Delete(DeleteArgs),

    /// List all customers
    List,

    /// Edit a customer
    Edit(EditArgs),
}

impl Command for CustomerCommand {
    fn execute(&self) -> anyhow::Result<()> {
        let connection = create_connection(Path::new(DATABASE_PATH));
        let customers = CustomerRepository::new(connection);
        match &self {
            CustomerCommand::Create(args) => create(customers, args.clone()),
            CustomerCommand::Show(args) => show(customers, args.id),
            CustomerCommand::Delete(args) => delete(customers, args.id),
            CustomerCommand::List => list(customers),
            CustomerCommand::Edit(args) => edit(customers, args.clone()),
        }
    }
}

fn create(mut customers: CustomerRepository, args: CreateArgs) -> anyhow::Result<()> {
    let new_customer = NewCustomer {
        name: args.name,
        surname: args.surname,
        email: args.email,
        phone: args.phone,
    };

    let new_customer = customers.create(&new_customer)?;
    println!("{:?}", new_customer);
    Ok(())
}

fn show(mut customers: CustomerRepository, customer_id: i32) -> anyhow::Result<()> {
    let customer = customers.read(customer_id)?.unwrap();
    println!("{:?}", customer);
    Ok(())
}

fn delete(mut customers: CustomerRepository, customer_id: i32) -> anyhow::Result<()> {
    let deleted_costumer = customers.delete(customer_id)?;
    println!("{:?}", deleted_costumer);
    Ok(())
}

fn list(mut customers: CustomerRepository) -> anyhow::Result<()> {
    for customer in customers.read_all()? {
        println!("{:?}", customer);
    }
    Ok(())
}

fn edit(mut customers: CustomerRepository, args: EditArgs) -> anyhow::Result<()> {
    let customer_update = UpdateCustomer {
        name: args.name,
        surname: args.surname,
        email: args.email,
        phone: args.phone,
    };

    let updated_customer = customers.update(args.id, &customer_update)?;
    println!("{:?}", updated_customer);
    Ok(())
}
