//! # dade
//! ![Test](https://github.com/odd12258053/dade/workflows/Test/badge.svg)
//! [![Crates.io](https://img.shields.io/crates/v/dade.svg)](https://crates.io/crates/dade)
//!
//! dade is data definition for Rust structures.
//!
//! For the easy handle of data, the following will support it.
//!
//! + Data validation.
//! + Data schema conforms JsonSchema.
//!
//!
//! ## Example
//! ### Basic
//! To define a model, You need the below module.
//!
//! ```rust
//! use dade::Model;
//! use dade_derive::model;
//! ```
//!
//! For example, define user-model.
//! ```rust
//! #[model]
//! struct User {
//!     #[field(ge = 1)]
//!     id: u64,
//!     #[field(min_length = 1, max_length = 100)]
//!     name: String,
//!     #[field(default = "en")]
//!     lang: String,
//!     #[field(min_length = 1, max_length = 255, default = null)]
//!     url: Option<String>,
//!     #[field(default = false)]
//!     verified: bool,
//! }
//! ```
//!
//! Then you create an instance of the model by the below.
//!
//! ```rust
//! let input = "{\"id\": 1, \"name\": \"James Smith\"}";
//! let user = User::parse(input).unwrap();
//! ```
//!
//! And you get a Json string for the instance by the below.
//! ```rust
//! let json_string = user.json(false);
//! // json_string = "{\"id\":1,\"name\":\"James Smith\",\"lang\":\"en\",\"url\":null,\"verified\":false}"
//! ```
//!
//! If you want to validate a value, you will get a schema that conforms JsonSchema, for the given model, by the below.
//!
//! ```rust
//! let schema = User::schema();
//! ```
//!
//! The schema is
//! ```json
//! {
//!   "$ref": "#/definitions/User",
//!   "definitions": {
//!     "User": {
//!       "title": "User",
//!       "type": "object",
//!       "properties": {
//!         "id": {
//!           "type": "integer",
//!           "title": "Id",
//!           "minimum": 1
//!         },
//!         "name": {
//!           "type": "string",
//!           "title": "Name",
//!           "minLength": 1,
//!           "maxLength": 100
//!         },
//!         "lang": {
//!           "type": "string",
//!           "title": "Lang",
//!           "default": "en"
//!         },
//!         "url": {
//!           "type": "string",
//!           "title": "Url",
//!           "default": null,
//!           "minLength": 1,
//!           "maxLength": 255
//!         },
//!         "verified": {
//!           "type": "boolean",
//!           "title": "Verified",
//!           "default": false
//!         }
//!       },
//!       "required": ["id", "name"]
//!     }
//!   }
//! }
//! ```
//!
//!
//! ### Advance
//! * If you want to bind other name
//! ```rust
//! #[model]
//! struct User {
//!     id: u64,
//!     #[field(alias = "FirstName")]
//!     first_name: String,
//!     #[field(alias = "LastName")]
//!     last_name: String,
//! }
//! ```
//!
//! * If you need a nested model
//!
//! ```rust
//! #[model]
//! struct Name {
//!     first_name: String,
//!     last_name: String,
//! }
//!
//! #[model]
//! struct User {
//!     id: u64,
//!     name: Name,
//! }
//! ```
//!
//! * If you need a self-reference model
//!
//! ```rust
//! #[model]
//! struct Item {
//!     id: u64,
//!     name: String,
//!     value: u128,
//!     related_items: Option<Vec<Box<Item>>>,
//! }
//! ```
use std::collections::BTreeMap;

pub use dade_macro::model;

mod error;
pub use crate::error::{Error, ErrorType, Result};

mod json;
pub use crate::json::{FromJsonValue, JsonValue, Number, ToJsonValue};

mod dump;
pub use crate::dump::JsonDumper;

mod load;
pub use crate::load::JsonLoader;

mod schema;
pub use crate::schema::RegisterSchema;

mod stream;
pub use crate::stream::{SliceBytes, StrStream, Stream};

/// A trait for converting string to titlecased.
pub trait ToTitle {
    /// Returns a titlecased string.
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

/// Dump a JsonValue to string.
///
/// For example,
///
/// ```rust
/// use dade::{JsonValue, json_dump};
///
/// let ret = json_dump(&JsonValue::Null, false);
/// assert_eq!(ret, "null");
///
/// let ret = json_dump(&JsonValue::Bool(true), false);
/// assert_eq!(ret, "true");
/// ```
pub fn json_dump(json: &JsonValue, ensure_ascii: bool) -> String {
    JsonDumper::new(ensure_ascii).dump(json)
}

/// Load from string to JsonValue.
///
/// For example,
///
/// ```rust
/// use dade::{JsonValue, json_load};
///
/// let ret = json_load("null");
/// // ret is JsonValue::Null.
///
/// let ret = json_load("true");
/// // ret is JsonValue::Bool(true).
/// ```
pub fn json_load(json: &str) -> Result<JsonValue> {
    JsonLoader::from(json).load()
}

/// A trait defines the format to handle a model.
///
/// This trait is efficiently handled data corresponding to an implemented struct.
pub trait Model {
    /// Convert a JSON string to a struct that implemented this trait.
    /// If the JSON string is invalid, the return is Err.
    /// Also, if valid, the return is Ok that contains an instance.
    fn parse(json: &str) -> Result<Self>
    where
        Self: Sized;
    /// Convert bytes to a struct that implemented this trait.
    /// If the JSON string is invalid, the return is Err.
    /// Also, if valid, the return is Ok that contains an instance.
    fn parse_bytes(bytes: &[u8]) -> Result<Self>
    where
        Self: Sized;
    /// Dump a JSON string from the instance.
    fn json(&self, ensure_ascii: bool) -> String;
    /// Export a JSON Schema with a model.
    fn schema() -> String;
}

impl<T: ToJsonValue + FromJsonValue + RegisterSchema> Model for T {
    fn parse(json: &str) -> Result<Self> {
        FromJsonValue::from_json_value(&JsonLoader::from(json).load()?)
    }
    fn parse_bytes(bytes: &[u8]) -> Result<Self> {
        FromJsonValue::from_json_value(&JsonLoader::from(bytes).load()?)
    }
    fn json(&self, ensure_ascii: bool) -> String {
        json_dump(&ToJsonValue::to_json_value(self), ensure_ascii)
    }
    fn schema() -> String {
        let mut defs = BTreeMap::new();
        let json_value = <T as RegisterSchema>::register_schema(&mut defs);
        match json_value {
            JsonValue::Object(ref dict) => {
                if let Some(JsonValue::String(def_name)) = dict.get(&"$ref".to_string()) {
                    return JsonValue::Object(BTreeMap::from([
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
