use super::CRUD;

use crate::{
    cli::BusinessCommand,
    database::{
        models::{Business, Config, BUSINESS_PRIMARY_KEY, CONFIG_PRIMARY_KEY},
        FactureDatabase, YamlAble,
    },
};
use anyhow::Result;

impl YamlAble for Business {}
impl CRUD for Business {}

pub fn handle_business_command(command: &BusinessCommand, database: FactureDatabase) -> Result<()> {
    match command {
        BusinessCommand::Init => {
            // Init config
            if database.exists::<Config>(CONFIG_PRIMARY_KEY)? {
                println!("Config already exists");
            } else {
                let config = Config::default();
                Config::create(&database, &config)?;
            };

            // Init business
            if database.exists::<Business>(BUSINESS_PRIMARY_KEY)? {
                println!("Business already exists");
            } else {
                let business = Business::default();
                Business::create(&database, &business)?;
            }
        }
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
