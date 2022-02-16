extern crate indexmap;

mod error;
pub use crate::error::{Error, Result};

mod json;
pub use crate::json::{FromJsonValue, JsonValue, ToJsonValue};

mod dump;
pub use crate::dump::JsonDumper;

mod load;
pub use crate::load::JsonLoader;

pub fn json_dump(json: &JsonValue) -> String {
    JsonDumper::dump(json)
}

pub fn json_load(json: &str) -> Result<JsonValue> {
    JsonLoader::new(json).load()
}

pub trait Model {
    fn parse(json: &str) -> Result<Self>
    where
        Self: Sized;
    fn json(&self) -> String;
}

impl<T: ToJsonValue + FromJsonValue> Model for T {
    fn parse(json: &str) -> Result<Self> {
        FromJsonValue::from_json_value(&json_load(json)?)
    }
    fn json(&self) -> String {
        ToJsonValue::to_json_value(self).to_string()
    }
}
