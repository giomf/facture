use super::CONFIG_TABLE_NAME;
use crate::{
    commands::CRUD,
    database::{Model, YamlAble},
};
use serde::{Deserialize, Serialize};

const CUSTOMER_PREFIX_DEFAULT: &str = "C";
const INVOICE_PREFIX_DEFAULT: &str = "I";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub customer_prefix: String,
    pub customer_counter: usize,
    pub invoice_prefix: String,
    pub invoice_counter: usize,
}

impl YamlAble for Config {}
impl CRUD for Config {}

impl Model for Config {
    fn table() -> String {
        CONFIG_TABLE_NAME.to_owned()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            customer_prefix: CUSTOMER_PREFIX_DEFAULT.to_owned(),
            customer_counter: 1,
            invoice_prefix: INVOICE_PREFIX_DEFAULT.to_owned(),
            invoice_counter: 1,
        }
    }
}
