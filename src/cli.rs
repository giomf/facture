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
    #[command(subcommand)]
    Customer(ItemCommand),
    #[command(subcommand)]
    Invoice(ItemCommand),
}

#[derive(Subcommand, Debug)]
pub enum ItemCommand {
    List,
    Add,
    Remove,
    Edit,
}
