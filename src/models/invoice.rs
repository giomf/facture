use bon::Builder;
use chrono::NaiveDate;
use native_db::*;
use native_model::{native_model, Model};
use serde::{Deserialize, Serialize};

use super::{uuid, YamlAble};

#[derive(Serialize, Deserialize, Debug, Builder, Clone, PartialEq, Eq)]
#[native_model(id = 2, version = 1)]
#[native_db]
#[builder(on(String, into))]
pub struct Invoice {
    #[primary_key]
    #[builder(default = uuid())]
    pub uuid: String,
    pub date: NaiveDate,
    pub customer: String,
    pub items: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug, Builder, Clone, PartialEq, Eq)]
pub struct Item {
    pub description: String,
    pub price: u32,
    pub quantity: Option<u32>,
}

impl YamlAble for Invoice {}
