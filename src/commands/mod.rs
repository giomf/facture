pub mod customer;
pub mod invoice;

use crate::{cli::ItemCommand, database::FactureDatabase};
use anyhow::Result;

pub fn handle_customer_command(command: &ItemCommand, database: &FactureDatabase) -> Result<()> {
    match command {
        ItemCommand::List => customer::list(database)?,
        ItemCommand::Add => customer::add(database)?,
        ItemCommand::Remove => customer::remove(database)?,
        ItemCommand::Edit => customer::edit(database)?,
    }
    Ok(())
}

pub fn handle_invoice_command(command: &ItemCommand, database: &FactureDatabase) -> Result<()> {
    match command {
        ItemCommand::List => invoice::list(database)?,
        ItemCommand::Add => invoice::add(database)?,
        ItemCommand::Remove => invoice::remove(database)?,
        ItemCommand::Edit => invoice::edit(database)?,
    }
    Ok(())
}
