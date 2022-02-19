pub use indexmap::IndexMap;

mod error;
pub use crate::error::{Error, Result};

mod json;
pub use crate::json::{FromJsonValue, JsonValue, Number, ToJsonValue};

mod dump;
pub use crate::dump::JsonDumper;

mod load;
pub use crate::load::JsonLoader;

mod schema;
pub use crate::schema::RegisterSchema;

pub trait ToTitle {
    fn to_title(&self) -> String;
}

impl ToTitle for str {
    fn to_title(&self) -> String {
        if self.is_empty() {
            self.to_string()
        } else {
            let mut buffer = String::with_capacity(self.len());
            let mut chars = self.chars();
            if let Some(c) = chars.next() {
                buffer.push_str(c.to_uppercase().to_string().as_str());
            }
            for c in chars {
                buffer.push(c);
            }
            buffer
        }
    }
}

pub fn json_dump(json: &JsonValue, ensure_ascii: bool) -> String {
    JsonDumper::new(ensure_ascii).dump(json)
}

pub fn json_load(json: &str) -> Result<JsonValue> {
    JsonLoader::new(json).load()
}

pub trait Model {
    fn parse(json: &str) -> Result<Self>
    where
        Self: Sized;
    fn json(&self, ensure_ascii: bool) -> String;
    fn schema() -> String;
}

impl<T: ToJsonValue + FromJsonValue + RegisterSchema> Model for T {
    fn parse(json: &str) -> Result<Self> {
        FromJsonValue::from_json_value(&json_load(json)?)
    }
    fn json(&self, ensure_ascii: bool) -> String {
        json_dump(&ToJsonValue::to_json_value(self), ensure_ascii)
    }
    fn schema() -> String {
        let mut defs = IndexMap::new();
        let json_value = <T as RegisterSchema>::register_schema(&mut defs);
        match json_value {
            JsonValue::Object(ref dict) => {
                if let Some(JsonValue::String(def_name)) = dict.get(&"$ref".to_string()) {
                    return JsonValue::Object(IndexMap::from([
                        ("$ref".to_string(), JsonValue::String(def_name.to_string())),
                        ("definitions".to_string(), JsonValue::Object(defs)),
                    ]))
                    .to_string();
                }
                json_value.to_string()
            }
            _ => json_value.to_string(),
        }
    }
}
