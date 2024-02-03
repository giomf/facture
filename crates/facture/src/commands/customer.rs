use super::Command;
use clap::{Args, Subcommand};
use database::{
    create_connection,
    customer::{Customers, NewCustomer},
    Repository, DATABASE_PATH,
};

#[derive(Args, Clone, Debug)]
pub struct CreateArgs {
    pub name: String,
    pub surname: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Args, Clone, Debug)]
pub struct DeleteArgs {
    pub id: i32,
}

#[derive(Debug, Args)]
pub struct EditArgs {
    pub id: i32,
    pub name: Option<String>,
    pub surname: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum CustomerCommand {
    /// Create a new customer
    Create(CreateArgs),
    /// Delete a customer
    Delete(DeleteArgs),
    /// List all customers
    List,
    /// Edit a customer
    Edit(EditArgs),
}

impl Command for CustomerCommand {
    fn execute(&self) -> anyhow::Result<()> {
        let connection = create_connection(DATABASE_PATH);
        let customers = Customers::new(connection);
        match &self {
            CustomerCommand::Create(args) => create(customers, args.clone()),
            CustomerCommand::Delete(args) => delete(customers, args.clone()),
            CustomerCommand::List => list(customers),
            CustomerCommand::Edit(_) => todo!(),
        }
    }
}

fn create(mut customers: Customers, args: CreateArgs) -> anyhow::Result<()> {
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

fn delete(mut customers: Customers, args: DeleteArgs) -> anyhow::Result<()> {
    let deleted_costumer = customers.delete(args.id)?;
    println!("{:?}", deleted_costumer);
    Ok(())
}

fn list(mut customers: Customers) -> anyhow::Result<()> {
    for customer in customers.read_all()? {
        println!("{:?}", customer);
    }
    Ok(())
}
