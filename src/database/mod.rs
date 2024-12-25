pub mod models;

use serde::{de::DeserializeOwned, Serialize};
use std::{fs, io, path::PathBuf};

pub type Result<T> = std::result::Result<T, Error>;

pub trait Model: Serialize + DeserializeOwned + YamlAble {
    fn table() -> String;
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O error")]
    IO(#[from] io::Error),

    #[error("Deserialization or Serialization error")]
    DeOrSerialization(#[from] serde_yml::Error),

    #[error("Table \"{0}\" not found")]
    TableNotFound(String),

    #[error("Key \"{0}\" already exists")]
    KeyAlreadyExists(String),

    #[error("Key \"{0}\" not found")]
    KeyNotFound(String),
}

#[derive(Debug, Clone)]
pub struct FilesystemDatabase {
    path: PathBuf,
}

pub trait YamlAble: Serialize + DeserializeOwned {
    fn to_yaml(&self) -> Result<String> {
        let yaml = serde_yml::to_string(&self)?;
        Ok(yaml)
    }

    fn from_yaml(yaml: &str) -> Result<Self> {
        let object: Self = serde_yml::from_str(yaml)?;
        Ok(object)
    }
}

impl FilesystemDatabase {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn define<T: Model>(&self) -> Result<()> {
        let path = self.path.join(T::table());
        if !path.exists() {
            fs::create_dir_all(path)?;
        }
        Ok(())
    }

    pub fn create<T: Model>(&self, key: &str, value: T) -> Result<()> {
        let table = self.path.join(T::table());
        if !table.exists() {
            return Err(Error::TableNotFound(T::table()));
        }

        let path = table.join(format!("{key}.yaml"));
        if path.exists() {
            return Err(Error::KeyAlreadyExists(key.to_owned()));
        }

        let yaml = value.to_yaml()?;
        fs::write(path, yaml)?;
        Ok(())
    }

    pub fn read<T: Model>(&self, key: &str) -> Result<T> {
        let table = self.path.join(T::table());
        if !table.exists() {
            return Err(Error::TableNotFound(T::table()));
        }

        let path = table.join(format!("{key}.yaml"));
        if !path.exists() {
            return Err(Error::KeyNotFound(key.to_owned()));
        }

        let value = fs::read_to_string(path)?;
        let value = T::from_yaml(&value)?;
        Ok(value)
    }

    pub fn update<T: Model>(&self, key: &str, value: T) -> Result<()> {
        let table = self.path.join(T::table());
        if !table.exists() {
            return Err(Error::TableNotFound(T::table()));
        }

        let path = table.join(format!("{key}.yaml"));
        if !path.exists() {
            return Err(Error::KeyNotFound(key.to_owned()));
        }

        let yaml = value.to_yaml()?;
        fs::write(path, yaml)?;
        Ok(())
    }

    pub fn delete<T: Model>(&self, key: &str) -> Result<()> {
        let table = self.path.join(T::table());
        if !table.exists() {
            return Err(Error::TableNotFound(T::table()));
        }

        let path = table.join(format!("{key}.yaml"));
        if !path.exists() {
            return Err(Error::KeyNotFound(key.to_owned()));
        }

        fs::remove_file(path)?;

        Ok(())
    }

    pub fn exists<T: Model>(&self, key: &str) -> Result<bool> {
        let table = self.path.join(T::table());
        if !table.exists() {
            return Err(Error::TableNotFound(T::table()));
        }

        let path = table.join(format!("{key}.yaml"));
        Ok(path.exists())
    }

    pub fn read_all<T: Model>(&self) -> Result<Vec<T>> {
        let table = self.path.join(T::table());
        if !table.exists() {
            return Err(Error::TableNotFound(T::table()));
        }

        let mut values = Vec::default();
        for entry in fs::read_dir(table)? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                let value = fs::read_to_string(entry.path())?;
                let value = T::from_yaml(&value)?;
                values.push(value);
            }
        }
        Ok(values)
    }
}
