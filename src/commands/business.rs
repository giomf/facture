use super::CRUD;
use crate::{filesystem_database::YamlAble, models::business::Business};

impl YamlAble for Business {}
impl CRUD for Business {}
