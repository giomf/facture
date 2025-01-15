use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about = "Facture is a small customer & invoice database that lets you render invoices to pdf", long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Subcommands of the application
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize facture
    Init,
    /// Manipulate customers
    #[command(subcommand)]
    Customer(CustomerCommand),
    /// Manipulate invoices
    #[command(subcommand)]
    Invoice(InvoiceCommand),
    #[command(subcommand)]
    /// Manipulate own business
    Business(BusinessCommand),
    /// Manipulate configuration
    #[command(subcommand)]
    Config(ConfigCommand),
}

#[derive(Subcommand, Debug)]
pub enum CustomerCommand {
    /// Add a customer
    Add,
    /// Edit a customer
    Edit,
    /// List all cusomters
    List,
    /// Remove a cusomter
    Remove,
    /// Show a customer
    Show,
}

#[derive(Subcommand, Debug)]
pub enum InvoiceCommand {
    /// Add an invoice
    Add,
    /// Edit an invoice
    Edit,
    /// List all invoices
    List,
    /// Remove a invoice
    Remove,
    /// Show a invoice
    Show,
    /// Render a invoice to pdf
    Render,
}

#[derive(Subcommand, Debug)]
pub enum BusinessCommand {
    /// Edit business
    Edit,
    /// Show business
    Show,
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommand {
    /// Edit configuration
    Edit,
    /// Show configuration
    Show,
}
