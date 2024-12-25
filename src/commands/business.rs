use crate::database::{models::business::Business, YamlAble};

use super::CRUD;

impl YamlAble for Business {}
impl CRUD for Business {}
