pub mod business;
pub mod customer;
pub mod invoice;

use crate::{
    cli::ConfigCommand,
    database::{models::config::Config, FilesystemDatabase, Model, YamlAble},
    ui::{self, TableAble},
};
use anyhow::Result;
use std::{env, fs, process::Command};
use tempfile::Builder;

const BUSINESS_KEY: &str = "business";
const CONFIG_KEY: &str = "config";

pub trait ListAble: Model + TableAble {
    fn list(database: FilesystemDatabase) -> Result<()> {
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

pub trait CRUD: Clone + Model {
    fn create(database: &FilesystemDatabase, object: &Self, key: &str) -> Result<()> {
        let new_object = edit_object_in_temp_file(object)?;
        database.create(key, new_object.clone())?;
        let new_object_yaml = new_object.to_yaml()?;
        println!("\n{new_object_yaml}");
        Ok(())
    }

    fn show(object: &Self) -> Result<()> {
        let object_yaml = object.to_yaml()?;
        println!("{object_yaml}");
        Ok(())
    }

    fn edit(database: &FilesystemDatabase, object: &Self, key: &str) -> Result<()> {
        let new_object = edit_object_in_temp_file(object)?;
        database.update(key, new_object.clone())?;
        let new_object_yaml = new_object.to_yaml()?;
        println!("\n{new_object_yaml}");
        Ok(())
    }

    fn remove(database: &FilesystemDatabase, key: &str) -> Result<()> {
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

pub fn handle_config_command(command: &ConfigCommand, database: FilesystemDatabase) -> Result<()> {
    match command {
        ConfigCommand::Edit => {
            let config = database.read::<Config>(CONFIG_KEY)?;
            Config::edit(&database, &config, CONFIG_KEY)?;
        }
        ConfigCommand::Show => {
            let config = database.read::<Config>(CONFIG_KEY)?;
            Config::show(&config)?;
        }
    }

    Ok(())
}
