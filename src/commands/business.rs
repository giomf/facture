use super::CRUD;

use crate::{
    cli::BusinessCommand,
    database::{
        models::{Business, BUSINESS_PRIMARY_KEY},
        FactureDatabase, YamlAble,
    },
};
use anyhow::Result;

impl YamlAble for Business {}
impl CRUD for Business {}

pub fn handle_business_command(command: &BusinessCommand, database: FactureDatabase) -> Result<()> {
    match command {
        BusinessCommand::Edit => {
            let business = database.read::<Business>(BUSINESS_PRIMARY_KEY)?;
            Business::edit(&database, &business, BUSINESS_PRIMARY_KEY)?;
        }
        BusinessCommand::Show => {
            let business = database.read::<Business>(BUSINESS_PRIMARY_KEY)?;
            Business::show(&business)?;
        }
    }
    Ok(())
}
