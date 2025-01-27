use crate::{commands::CRUD, database::YamlAble};

use native_db::{native_db, ToKey};
use native_model::{native_model, Model};
use serde::{Deserialize, Serialize};

pub const PRIMARY_KEY: &str = "CONFIG";
const CUSTOMER_TEMPLATE_DEFAULT: &str = "K{{ counter }}";
const INVOICE_TEMPLATE_DEFAULT: &str = "R{{ year }}-{{ counter }}";

pub type Config = v1::Config;

pub mod v1 {
    use super::*;

    #[native_db(primary_key(primary_key -> String))]
    #[native_model(id = 4, version = 1)]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Config {
        pub customer_template: String,
        pub customer_counter: usize,
        pub invoice_template: String,
        pub invoice_counter: usize,
    }
}

impl YamlAble for Config {}
impl CRUD for Config {}

impl Default for Config {
    fn default() -> Self {
        Self {
            customer_template: CUSTOMER_TEMPLATE_DEFAULT.to_owned(),
            customer_counter: 1,
            invoice_template: INVOICE_TEMPLATE_DEFAULT.to_owned(),
            invoice_counter: 1,
        }
    }
}

impl Config {
    fn primary_key(&self) -> String {
        PRIMARY_KEY.to_owned()
    }
}
