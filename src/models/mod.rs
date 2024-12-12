pub mod business;
pub mod customer;
pub mod invoice;

use anyhow::Result;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use uuid::Uuid;

pub trait YamlAble: Serialize + DeserializeOwned {
    fn to_yaml(&self) -> Result<String> {
        let yaml = serde_yaml::to_string(&self)?;
        Ok(yaml)
    }

    fn from_yaml(yaml: &str) -> Result<Self> {
        let object: Self = serde_yaml::from_str(yaml)?;
        Ok(object)
    }
}

#[derive(Serialize, Default, Deserialize, Debug, Clone)]
pub struct Address {
    pub country: String,
    pub city: String,
    pub postal_code: String,
    pub street: String,
    pub number: String,
}

#[derive(Serialize, Default, Deserialize, Debug, Clone)]
pub struct Contact {
    pub name: String,
    pub surname: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

fn uuid_v7() -> String {
    Uuid::now_v7().to_string()
}
