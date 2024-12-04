use bon::Builder;
use native_db::*;
use native_model::{native_model, Model};
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use uuid::Uuid;

pub static MODELS: LazyLock<Models> = LazyLock::new(|| {
    let mut models = Models::new();
    models.define::<Customer>().unwrap();
    models.define::<Invoice>().unwrap();
    models
});

#[derive(Serialize, Deserialize, Debug, Builder, Clone, PartialEq, Eq)]
#[native_model(id = 1, version = 1)]
#[native_db]
#[builder(on(String, into))]
pub struct Customer {
    #[builder(default = uuid())]
    #[primary_key]
    pub uuid: String,
    pub name: String,
    pub surname: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Builder, Clone, PartialEq, Eq)]
#[native_model(id = 2, version = 1)]
#[native_db]
#[builder(on(String, into))]
pub struct Invoice {
    #[primary_key]
    #[builder(default = uuid())]
    pub uuid: String,
    pub customer_id: i32,
}

fn uuid() -> String {
    Uuid::now_v7().to_string()
}
