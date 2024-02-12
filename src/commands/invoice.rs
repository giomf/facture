use std::path::Path;

use super::Command;
use crate::database::{
    create_connection,
    customer::CustomerRepository,
    invoice::{InvoiceRepository, NewInvoice},
    Repository, DATABASE_PATH,
};
use clap::{Args, Subcommand};

#[derive(Args, Clone, Debug)]
pub struct CreateArgs {
    /// Customer ID
    #[arg(long)]
    pub customer_id: i32,
}

#[derive(Args, Clone, Debug)]
pub struct DeleteArgs {
    /// Invoice ID
    pub id: i32,
}

#[derive(Args, Clone, Debug)]
pub struct ListArgs {
    /// Customer ID
    #[arg(long)]
    pub customer_id: i32,
}

#[derive(Debug, Subcommand)]
pub enum InvoiceCommand {
    /// Create a new invoice
    Create(CreateArgs),

    /// Delete a invoice
    Delete(DeleteArgs),

    /// List all invoices related to a customer
    List(ListArgs),
}

impl Command for InvoiceCommand {
    fn execute(&self) -> anyhow::Result<()> {
        let database_path = Path::new(DATABASE_PATH);
        let customers_connection = create_connection(database_path);
        let invoices_connection = create_connection(database_path);
        let customers = CustomerRepository::new(customers_connection);
        let invoices = InvoiceRepository::new(invoices_connection);

        match &self {
            InvoiceCommand::Create(args) => create(customers, invoices, args.customer_id),
            InvoiceCommand::Delete(args) => delete(invoices, args.id),
            InvoiceCommand::List(args) => list(customers, invoices, args.customer_id),
        }
    }
}

fn create(
    mut customers: CustomerRepository,
    mut invoices: InvoiceRepository,
    customer_id: i32,
) -> anyhow::Result<()> {
    if !customers.exists(customer_id)? {
        anyhow::bail!("Customer {customer_id} does not exists")
    }
    let new_customer = NewInvoice { customer_id };
    let new_invoice = invoices.create(&new_customer)?;
    println!("{:?}", new_invoice);
    Ok(())
}

fn delete(mut invoices: InvoiceRepository, invoice_id: i32) -> anyhow::Result<()> {
    if !invoices.exists(invoice_id)? {
        anyhow::bail!("Invoice {invoice_id} does not exists")
    }
    let deleted_invoice = invoices.delete(invoice_id)?;
    println!("{:?}", deleted_invoice);
    Ok(())
}

fn list(
    mut customers: CustomerRepository,
    mut invoices: InvoiceRepository,
    customer_id: i32,
) -> anyhow::Result<()> {
    if !customers.exists(customer_id)? {
        anyhow::bail!("Customer {customer_id} does not exists")
    }

    for invoice in invoices.read_all()? {
        println!("{:?}", invoice);
    }

    Ok(())
}
