pub mod models;

use anyhow::{anyhow, Result};
use models::v1;
use native_db::*;
use serde::{de::DeserializeOwned, Serialize};
use std::{path::Path, sync::LazyLock};

pub const DATABASE_PATH: &str = "./facture.db";

pub static MODELS: LazyLock<Models> = LazyLock::new(|| {
    let mut models = Models::new();
    // It's a good practice to define the models by specifying the version
    models.define::<v1::Customer>().unwrap();
    models.define::<v1::Invoice>().unwrap();
    models.define::<v1::Business>().unwrap();
    models.define::<v1::Config>().unwrap();
    models
});

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

pub struct FactureDatabase<'a> {
    _builder: Builder,
    database: Database<'a>,
}

impl<'a> FactureDatabase<'a> {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let builder = Builder::new();
        let database = builder.create(&MODELS, path.as_ref())?;
        let database = Self {
            _builder: builder,
            database,
        };
        Ok(database)
    }

    pub fn create<T: ToInput + Clone>(&self, item: T) -> Result<()> {
        let rw = self.database.rw_transaction()?;
        rw.insert(item)?;
        rw.commit()?;
        Ok(())
    }

    pub fn update<T: ToInput>(&self, uuid: &str, item: T) -> Result<()> {
        let rw = self.database.rw_transaction()?;
        let old: T = self.read(uuid)?;

        rw.update(old, item)?;
        rw.commit()?;
        Ok(())
    }

    pub fn exists<T: ToInput>(&self, uuid: &str) -> Result<bool> {
        let r = self.database.r_transaction()?;
        let result: Option<T> = r.get().primary(uuid)?;
        Ok(result.is_some())
    }

    pub fn read<T: ToInput>(&self, uuid: &str) -> Result<T> {
        let r = self.database.r_transaction()?;
        let result: T = r
            .get()
            .primary(uuid)?
            .ok_or_else(|| anyhow!("{uuid} not found"))?;

        Ok(result)
    }

    pub fn read_all<T: ToInput>(&self) -> Result<Vec<T>> {
        let r = self.database.r_transaction()?;
        let result = r.scan().primary()?.all()?.filter_map(Result::ok).collect();
        Ok(result)
    }

    pub fn delete<T: ToInput>(&self, key: &str) -> Result<()> {
        let rw = self.database.rw_transaction()?;
        let item: T = self.read(key)?;
        rw.remove(item)?;
        rw.commit()?;
        Ok(())
    }
}
