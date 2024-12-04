mod cli;
mod database;
mod models;

use clap::Parser;
use cli::Cli;
use database::{FactureDatabase, DATABASE_PATH};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        _ => todo!(),
    }
    Ok(())
}
