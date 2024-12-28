mod cli;
mod commands;
mod database;
mod template;
mod ui;

use std::{path::PathBuf, str::FromStr};

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use commands::{
    business::handle_business_command, customer::handle_customer_command, handle_config_command,
    invoice::handle_invoice_command,
};
use database::FilesystemDatabase;

const DATABASE_PATH: &str = "database";

fn main() -> Result<()> {
    let cli = Cli::parse();
    let path = PathBuf::from_str(DATABASE_PATH)?;
    let database = FilesystemDatabase::new(path);

    match &cli.command {
        cli::Commands::Customer(command) => handle_customer_command(command, database)?,
        cli::Commands::Invoice(command) => handle_invoice_command(command, database)?,
        cli::Commands::Business(command) => handle_business_command(command, database)?,
        cli::Commands::Config(command) => handle_config_command(command, database)?,
    }
    Ok(())
}
