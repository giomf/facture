use super::{BUSINESS_KEY, CONFIG_KEY, CRUD};
use crate::{
    cli::BusinessCommand,
    database::{
        models::{business::Business, config::Config, customer::Customer, invoice::Invoice},
        FilesystemDatabase, YamlAble,
    },
};
use anyhow::Result;

impl YamlAble for Business {}
impl CRUD for Business {}

pub fn handle_business_command(
    command: &BusinessCommand,
    database: FilesystemDatabase,
) -> Result<()> {
    match command {
        BusinessCommand::Init => {
            // Init database models
            database.define::<Business>()?;
            database.define::<Customer>()?;
            database.define::<Invoice>()?;
            database.define::<Config>()?;

            // Init config
            if database.exists::<Config>(CONFIG_KEY)? {
                println!("Config already exists");
            } else {
                let config = Config::default();
                Config::create(&database, &config, CONFIG_KEY)?;
            };

            // Init business
            if database.exists::<Business>(BUSINESS_KEY)? {
                println!("Business already exists");
            } else {
                let business = Business::default();
                Business::create(&database, &business, BUSINESS_KEY)?;
            }
        }
        BusinessCommand::Edit => {
            let business = database.read::<Business>(BUSINESS_KEY)?;
            Business::edit(&database, &business, BUSINESS_KEY)?;
        }
        BusinessCommand::Show => {
            let business = database.read::<Business>(BUSINESS_KEY)?;
            Business::show(&business)?;
        }
    }
    Ok(())
}
