pub mod customer;
pub mod invoice;

use anyhow::Result;
use customer::Customer;
use invoice::Invoice;
use native_db::Models;
use serde::Serialize;
use std::sync::LazyLock;
use uuid::Uuid;

pub static MODELS: LazyLock<Models> = LazyLock::new(|| {
    let mut models = Models::new();
    models.define::<Customer>().unwrap();
    models.define::<Invoice>().unwrap();
    models
});

pub trait YamlAble: Serialize {
    fn to_yaml(&self) -> Result<String> {
        let yaml = serde_yaml::to_string(&self)?;
        Ok(yaml)
    }
}

fn uuid() -> String {
    Uuid::now_v7().to_string()
}
