pub mod business;
pub mod customer;
pub mod invoice;

use crate::{
    cli::ConfigCommand,
    database::{
        models::{Business, Config, BUSINESS_PRIMARY_KEY, CONFIG_PRIMARY_KEY},
        FactureDatabase, YamlAble,
    },
    ui::{self, TableAble},
};
use anyhow::Result;
use chrono::{Datelike, Local};
use minijinja::{context, Environment};
use native_db::ToInput;
use std::{env, fs, process::Command};
use tempfile::Builder;

pub trait ListAble: TableAble + ToInput {
    fn list(database: FactureDatabase) -> Result<()> {
        let objects: Vec<Self> = database.read_all()?;
        if objects.is_empty() {
            println!("Nothing created yet.");
            return Ok(());
        }
        let header = Self::header();
        let rows = objects.into_iter().map(|object| object.row()).collect();
        let table = ui::table(header, rows);
        println!("{table}");
        Ok(())
    }
}

pub trait CRUD: Clone + YamlAble + ToInput {
    fn create(database: &FactureDatabase, object: &Self) -> Result<()> {
        let new_object = edit_object_in_temp_file(object)?;
        database.create(new_object.clone())?;
        let new_object_yaml = new_object.to_yaml()?;
        println!("\n{new_object_yaml}");
        Ok(())
    }

    fn show(object: &Self) -> Result<()> {
        let object_yaml = object.to_yaml()?;
        println!("{object_yaml}");
        Ok(())
    }

    fn edit(database: &FactureDatabase, object: &Self, key: &str) -> Result<()> {
        let new_object = edit_object_in_temp_file(object)?;
        database.update(key, new_object.clone())?;
        let new_object_yaml = new_object.to_yaml()?;
        println!("\n{new_object_yaml}");
        Ok(())
    }

    fn remove(database: &FactureDatabase, key: &str) -> Result<()> {
        database.delete::<Self>(key)?;
        Ok(())
    }
}

fn edit_object_in_temp_file<T: YamlAble>(object: &T) -> Result<T> {
    let object_yaml = object.to_yaml()?;
    let temp_file = Builder::new()
        .prefix("facture_")
        .suffix(".yaml")
        .tempfile()?;
    fs::write(temp_file.path(), object_yaml)?;
    let editor = env::var("EDITOR")?;
    Command::new(editor).arg(temp_file.path()).status()?;
    let object_yaml = fs::read_to_string(temp_file.path())?;
    let new_object = T::from_yaml(&object_yaml)?;
    Ok(new_object)
}

pub fn handle_config_command(command: &ConfigCommand, database: FactureDatabase) -> Result<()> {
    match command {
        ConfigCommand::Edit => {
            let config = database.read::<Config>(CONFIG_PRIMARY_KEY)?;
            Config::edit(&database, &config, CONFIG_PRIMARY_KEY)?;
        }
        ConfigCommand::Show => {
            let config = database.read::<Config>(CONFIG_PRIMARY_KEY)?;
            Config::show(&config)?;
        }
    }

    Ok(())
}

pub fn handle_init_command(database: FactureDatabase) -> Result<()> {
    // Init config
    if database.exists::<Config>(CONFIG_PRIMARY_KEY)? {
        println!("Config already exists... skipping!");
    } else {
        println!("Creating config...");
        let config = Config::default();
        Config::create(&database, &config)?;
    };

    // Init business
    if database.exists::<Business>(BUSINESS_PRIMARY_KEY)? {
        println!("Business already exists... skipping!");
    } else {
        println!("Creating Business...");
        let business = Business::default();
        Business::create(&database, &business)?;
    }

    Ok(())
}

pub fn render_id_template(template: &str, counter: &str) -> Result<String> {
    let env = Environment::new();
    let date = Local::now().date_naive();
    let id = env.render_str(template,
        context!(year => date.year().to_string(), month => date.month().to_string(), day => date.month().to_string(), counter => counter))?;
    Ok(id)
}
