use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about = "Divera reports", long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    /// Config path
    #[arg(global = true, short, long)]
    pub config: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

/// Subcommands of the application
#[derive(Subcommand, Debug)]
pub enum Commands {
    Init,
    #[command(subcommand)]
    Customer(CustomerCommand),
    #[command(subcommand)]
    Invoice(InvoiceCommand),
    #[command(subcommand)]
    Business(BusinessCommand),
    #[command(subcommand)]
    Config(ConfigCommand),
}

#[derive(Subcommand, Debug)]
pub enum CustomerCommand {
    Add,
    Edit,
    List,
    Remove,
    Show,
}

#[derive(Subcommand, Debug)]
pub enum InvoiceCommand {
    Add,
    Edit,
    List,
    Remove,
    Show,
    Render,
}

#[derive(Subcommand, Debug)]
pub enum BusinessCommand {
    Edit,
    Show,
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommand {
    Edit,
    Show,
}
