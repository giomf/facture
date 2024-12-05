use std::path::Path;

use crate::models::MODELS;
use anyhow::{anyhow, Result};
use native_db::{Builder, Database, ToInput};

pub const DATABASE_PATH: &str = "./facture.db";

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

    pub fn _open_in_memory() -> Result<Self> {
        let builder = Builder::new();
        let database = builder.create_in_memory(&MODELS)?;
        let database = Self {
            _builder: builder,
            database,
        };
        Ok(database)
    }

    pub fn insert<T: ToInput + Clone>(&self, item: &T) -> Result<()> {
        let rw = self.database.rw_transaction()?;
        rw.insert(item.to_owned())?;
        rw.commit()?;
        Ok(())
    }

    pub fn update<T: ToInput + Clone>(&self, uuid: &str, item: &T) -> Result<()> {
        let rw = self.database.rw_transaction()?;
        let old: T = self
            .read(uuid)?
            .ok_or_else(|| anyhow!("{uuid} not found"))?;

        rw.update(old, item.to_owned())?;
        rw.commit()?;
        Ok(())
    }

    pub fn read<T: ToInput>(&self, uuid: &str) -> Result<Option<T>> {
        let r = self.database.r_transaction()?;
        let result: Option<T> = r.get().primary(uuid)?;
        Ok(result)
    }

    pub fn read_all<T: ToInput>(&self) -> Result<Vec<T>> {
        let r = self.database.r_transaction()?;
        let result = r.scan().primary()?.all()?.filter_map(Result::ok).collect();
        Ok(result)
    }

    pub fn remove<T: ToInput + Clone>(&self, item: &T) -> Result<()> {
        let rw = self.database.rw_transaction()?;
        rw.remove(item.clone())?;
        rw.commit()?;
        Ok(())
    }
}
