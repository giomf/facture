mod cli;
mod commands;
mod database;
mod models;
mod ui;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use commands::{handle_customer_command, handle_invoice_command};
use database::{FactureDatabase, DATABASE_PATH};

fn main() -> Result<()> {
    let cli = Cli::parse();
    let database = FactureDatabase::open(DATABASE_PATH)?;
    match &cli.command {
        cli::Commands::Customer(command) => handle_customer_command(command, &database)?,
        cli::Commands::Invoice(command) => handle_invoice_command(command, &database)?,
    }
    Ok(())
}
