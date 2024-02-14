pub mod customer;
pub mod invoice;
pub mod item;
use self::customer::CustomerCommand;
use self::invoice::InvoiceCommand;
use clap::Parser;

pub trait Command {
    fn execute(&self) -> anyhow::Result<()>;
}

#[derive(Parser)]
#[clap(author, version, about = "facture", long_about = None)]
#[clap(propagate_version = true)]
pub enum Cli {
    /// Manipulate customers
    #[command(subcommand)]
    Customer(CustomerCommand),

    /// Manipulate invoices
    #[command(subcommand)]
    Invoice(InvoiceCommand),
}
