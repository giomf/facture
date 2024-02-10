use super::Command;
use clap::{Args, Subcommand};
use database::{
    create_connection,
    invoice::{InvoiceRepository, NewInvoice},
    Repository, DATABASE_PATH,
};

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

#[derive(Debug, Subcommand)]
pub enum InvoiceCommand {
    /// Create a new invoice
    Create(CreateArgs),

    /// Delete a invoice
    Delete(DeleteArgs),

    /// List all invoices related to a customer
    List,
}

impl Command for InvoiceCommand {
    fn execute(&self) -> anyhow::Result<()> {
        let connection = create_connection(DATABASE_PATH);
        let invoices = InvoiceRepository::new(connection);
        match &self {
            InvoiceCommand::Create(args) => create(invoices, args.clone()),
            InvoiceCommand::Delete(args) => delete(invoices, args.clone()),
            InvoiceCommand::List => list(invoices),
        }
    }
}

fn create(mut invoices: InvoiceRepository, args: CreateArgs) -> anyhow::Result<()> {
    let new_customer = NewInvoice {
        customer_id: args.customer_id,
    };

    let new_invoice = invoices.create(&new_customer)?;
    println!("{:?}", new_invoice);
    Ok(())
}

fn delete(mut invoices: InvoiceRepository, args: DeleteArgs) -> anyhow::Result<()> {
    let deleted_invoice = invoices.delete(args.id)?;
    println!("{:?}", deleted_invoice);
    Ok(())
}

fn list(mut invoices: InvoiceRepository) -> anyhow::Result<()> {
    for invoice in invoices.read_all()? {
        println!("{:?}", invoice);
    }
    Ok(())
}
