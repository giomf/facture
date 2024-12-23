mod cli;
mod commands;
mod filesystem_database;
mod models;
mod ui;

use std::{path::PathBuf, str::FromStr};

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use commands::{
    handle_business_command, handle_config_command, handle_customer_command, handle_invoice_command,
};
use filesystem_database::FilesystemDatabase;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let path = PathBuf::from_str("./database")?;
    let database = FilesystemDatabase::new(path);
    database.define::<models::business::Business>()?;
    database.define::<models::customer::Customer>()?;
    database.define::<models::invoice::Invoice>()?;

    match &cli.command {
        cli::Commands::Customer(command) => handle_customer_command(command, database)?,
        cli::Commands::Invoice(command) => handle_invoice_command(command, database)?,
        cli::Commands::Business(command) => handle_business_command(command, database)?,
        cli::Commands::Config(command) => handle_config_command(command, database)?,
    }
    Ok(())
}
