use super::CRUD;
use crate::models::{business::Business, YamlAble};

impl YamlAble for Business {}
impl CRUD for Business {}
