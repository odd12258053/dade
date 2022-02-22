use std::collections::BTreeMap;
use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;

use crate::dump::JsonDumper;
use crate::error::{Error, Result};

pub struct Number {
    value: String,
}

impl Number {
    pub fn new(value: String) -> Self {
        Self { value }
    }
    pub fn parse<F: FromStr>(&self) -> std::result::Result<F, F::Err> {
        self.value.parse()
    }
    pub fn from<T: std::string::ToString>(val: T) -> Self {
        Self {
            value: val.to_string(),
        }
    }
}

impl ToString for Number {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

/// Types for conforms JSON.
pub enum JsonValue {
    /// Correspond for null in JSON.
    Null,
    /// Correspond for boolean in JSON.
    Bool(bool),
    /// Correspond for number in JSON.
    Number(Number),
    /// Correspond for string in JSON.
    String(String),
    /// Correspond for array in JSON.
    Array(Vec<JsonValue>),
    /// Correspond for object in JSON.
    Object(BTreeMap<String, JsonValue>),
}

impl ToString for JsonValue {
    fn to_string(&self) -> String {
        JsonDumper::new(false).dump(self)
    }
}

/// A trait defines as the format to convert data to an instance.
pub trait FromJsonValue: Sized {
    fn from_json_value(value: &JsonValue) -> Result<Self>;
}

impl FromJsonValue for () {
    fn from_json_value(value: &JsonValue) -> Result<Self> {
        match value {
            JsonValue::Null => Ok(()),
            _ => Err(Error::new_validate_err("expect `JsonValue::Null`")),
        }
    }
}

macro_rules! from_json_value_for_int {
    ( $( $i:ident ),* ) => {
        $(
            impl FromJsonValue for $i {
                fn from_json_value(value: &JsonValue) -> Result<Self> {
                    match value {
                        JsonValue::Number(num) => num
                            .value
                            .parse()
                            .map_err(|err: ParseIntError| Error::new_validate_err(err.to_string().as_str())),
                        _ => Err(Error::new_validate_err("expect `JsonValue::Number`")),
                    }
                }
            }
        )*
    };
}

from_json_value_for_int!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

macro_rules! from_json_value_for_float {
    ( $( $i:ident ),* ) => {
        $(
            impl FromJsonValue for $i {
                fn from_json_value(value: &JsonValue) -> Result<Self> {
                    match value {
                        JsonValue::Number(num) => num
                            .value
                            .parse()
                            .map_err(|err: ParseFloatError| Error::new_validate_err(err.to_string().as_str())),
                        _ => Err(Error::new_validate_err("expect `JsonValue::Number`")),
                    }
                }
            }
        )*
    };
}

from_json_value_for_float!(f32, f64);

impl FromJsonValue for String {
    fn from_json_value(value: &JsonValue) -> Result<Self> {
        match value {
            JsonValue::String(s) => Ok(s.to_string()),
            _ => Err(Error::new_validate_err("expect `JsonValue::String`")),
        }
    }
}

impl FromJsonValue for bool {
    fn from_json_value(value: &JsonValue) -> Result<Self> {
        match value {
            JsonValue::Bool(b) => Ok(*b),
            _ => Err(Error::new_validate_err("expect `JsonValue::Bool`")),
        }
    }
}

impl<T: FromJsonValue> FromJsonValue for Vec<T> {
    fn from_json_value(value: &JsonValue) -> Result<Self> {
        match value {
            JsonValue::Array(arr) => {
                let mut buffer = Vec::with_capacity(arr.len());
                for val in arr.iter() {
                    buffer.push(FromJsonValue::from_json_value(val)?);
                }
                Ok(buffer)
            }
            _ => Err(Error::new_validate_err("expect `JsonValue::Arrya`")),
        }
    }
}

impl<T: FromJsonValue> FromJsonValue for Option<T> {
    fn from_json_value(value: &JsonValue) -> Result<Self> {
        match value {
            JsonValue::Null => Ok(None),
            _ => Ok(Some(FromJsonValue::from_json_value(value)?)),
        }
    }
}

impl<T: FromJsonValue> FromJsonValue for Box<T> {
    fn from_json_value(value: &JsonValue) -> Result<Self> {
        Ok(Box::new(FromJsonValue::from_json_value(value)?))
    }
}

/// A trait defines as the format to get data from the instance.
pub trait ToJsonValue {
    fn to_json_value(&self) -> JsonValue;
}

impl ToJsonValue for () {
    fn to_json_value(&self) -> JsonValue {
        JsonValue::Null
    }
}

macro_rules! to_json_value_for_num {
    ( $( $i:ident ),* ) => {
        $(
            impl ToJsonValue for $i {
                fn to_json_value(&self) -> JsonValue {
                    JsonValue::Number(Number::new(self.to_string()))
                }
            }
        )*
    };
}

to_json_value_for_num!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

impl ToJsonValue for String {
    fn to_json_value(&self) -> JsonValue {
        JsonValue::String(self.to_string())
    }
}

impl ToJsonValue for bool {
    fn to_json_value(&self) -> JsonValue {
        JsonValue::Bool(*self)
    }
}

impl<T: ToJsonValue> ToJsonValue for Vec<T> {
    fn to_json_value(&self) -> JsonValue {
        JsonValue::Array(
            self.iter()
                .map(|val| ToJsonValue::to_json_value(val))
                .collect(),
        )
    }
}

impl<T: ToJsonValue> ToJsonValue for Option<T> {
    fn to_json_value(&self) -> JsonValue {
        match self {
            None => JsonValue::Null,
            Some(val) => ToJsonValue::to_json_value(val),
        }
    }
}

impl<T: ToJsonValue> ToJsonValue for Box<T> {
    fn to_json_value(&self) -> JsonValue {
        T::to_json_value(self)
    }
}
