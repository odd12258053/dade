use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;

use indexmap::IndexMap;

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
}

impl ToString for Number {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

pub enum JsonValue {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<JsonValue>),
    Object(IndexMap<String, JsonValue>),
}

impl ToString for JsonValue {
    fn to_string(&self) -> String {
        JsonDumper::dump(self)
    }
}

pub trait FromJsonValue: Sized {
    fn from_json_value(value: &JsonValue) -> Result<Self>;
}

impl FromJsonValue for () {
    fn from_json_value(value: &JsonValue) -> Result<Self> {
        match value {
            JsonValue::Null => Ok(()),
            _ => Err(Error::new("expect `JsonValue::Null`")),
        }
    }
}

impl FromJsonValue for u8 {
    fn from_json_value(value: &JsonValue) -> Result<Self> {
        match value {
            JsonValue::Number(num) => num
                .value
                .parse()
                .map_err(|err: ParseIntError| Error::new(err.to_string().as_str())),
            _ => Err(Error::new("expect `JsonValue::Number`")),
        }
    }
}

impl FromJsonValue for u16 {
    fn from_json_value(value: &JsonValue) -> Result<Self> {
        match value {
            JsonValue::Number(num) => num
                .value
                .parse()
                .map_err(|err: ParseIntError| Error::new(err.to_string().as_str())),
            _ => Err(Error::new("expect `JsonValue::Number`")),
        }
    }
}

impl FromJsonValue for u32 {
    fn from_json_value(value: &JsonValue) -> Result<Self> {
        match value {
            JsonValue::Number(num) => num
                .value
                .parse()
                .map_err(|err: ParseIntError| Error::new(err.to_string().as_str())),
            _ => Err(Error::new("expect `JsonValue::Number`")),
        }
    }
}

impl FromJsonValue for u64 {
    fn from_json_value(value: &JsonValue) -> Result<Self> {
        match value {
            JsonValue::Number(num) => num
                .value
                .parse()
                .map_err(|err: ParseIntError| Error::new(err.to_string().as_str())),
            _ => Err(Error::new("expect `JsonValue::Number`")),
        }
    }
}

impl FromJsonValue for u128 {
    fn from_json_value(value: &JsonValue) -> Result<Self> {
        match value {
            JsonValue::Number(num) => num
                .value
                .parse()
                .map_err(|err: ParseIntError| Error::new(err.to_string().as_str())),
            _ => Err(Error::new("expect `JsonValue::Number`")),
        }
    }
}

impl FromJsonValue for usize {
    fn from_json_value(value: &JsonValue) -> Result<Self> {
        match value {
            JsonValue::Number(num) => num
                .value
                .parse()
                .map_err(|err: ParseIntError| Error::new(err.to_string().as_str())),
            _ => Err(Error::new("expect `JsonValue::Number`")),
        }
    }
}

impl FromJsonValue for i8 {
    fn from_json_value(value: &JsonValue) -> Result<Self> {
        match value {
            JsonValue::Number(num) => num
                .value
                .parse()
                .map_err(|err: ParseIntError| Error::new(err.to_string().as_str())),
            _ => Err(Error::new("expect `JsonValue::Number`")),
        }
    }
}

impl FromJsonValue for i16 {
    fn from_json_value(value: &JsonValue) -> Result<Self> {
        match value {
            JsonValue::Number(num) => num
                .value
                .parse()
                .map_err(|err: ParseIntError| Error::new(err.to_string().as_str())),
            _ => Err(Error::new("expect `JsonValue::Number`")),
        }
    }
}

impl FromJsonValue for i32 {
    fn from_json_value(value: &JsonValue) -> Result<Self> {
        match value {
            JsonValue::Number(num) => num
                .value
                .parse()
                .map_err(|err: ParseIntError| Error::new(err.to_string().as_str())),
            _ => Err(Error::new("expect `JsonValue::Number`")),
        }
    }
}

impl FromJsonValue for i64 {
    fn from_json_value(value: &JsonValue) -> Result<Self> {
        match value {
            JsonValue::Number(num) => num
                .value
                .parse()
                .map_err(|err: ParseIntError| Error::new(err.to_string().as_str())),
            _ => Err(Error::new("expect `JsonValue::Number`")),
        }
    }
}

impl FromJsonValue for i128 {
    fn from_json_value(value: &JsonValue) -> Result<Self> {
        match value {
            JsonValue::Number(num) => num
                .value
                .parse()
                .map_err(|err: ParseIntError| Error::new(err.to_string().as_str())),
            _ => Err(Error::new("expect `JsonValue::Number`")),
        }
    }
}

impl FromJsonValue for isize {
    fn from_json_value(value: &JsonValue) -> Result<Self> {
        match value {
            JsonValue::Number(num) => num
                .value
                .parse()
                .map_err(|err: ParseIntError| Error::new(err.to_string().as_str())),
            _ => Err(Error::new("expect `JsonValue::Number`")),
        }
    }
}

impl FromJsonValue for f32 {
    fn from_json_value(value: &JsonValue) -> Result<Self> {
        match value {
            JsonValue::Number(num) => num
                .value
                .parse()
                .map_err(|err: ParseFloatError| Error::new(err.to_string().as_str())),
            _ => Err(Error::new("expect `JsonValue::Number`")),
        }
    }
}

impl FromJsonValue for f64 {
    fn from_json_value(value: &JsonValue) -> Result<Self> {
        match value {
            JsonValue::Number(num) => num
                .value
                .parse()
                .map_err(|err: ParseFloatError| Error::new(err.to_string().as_str())),
            _ => Err(Error::new("expect `JsonValue::Number`")),
        }
    }
}

impl FromJsonValue for String {
    fn from_json_value(value: &JsonValue) -> Result<Self> {
        match value {
            JsonValue::String(s) => Ok(s.to_string()),
            _ => Err(Error::new("expect `JsonValue::String`")),
        }
    }
}

impl FromJsonValue for bool {
    fn from_json_value(value: &JsonValue) -> Result<Self> {
        match value {
            JsonValue::Bool(b) => Ok(*b),
            _ => Err(Error::new("expect `JsonValue::Bool`")),
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
            _ => Err(Error::new("expect `JsonValue::Arrya`")),
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

pub trait ToJsonValue {
    fn to_json_value(&self) -> JsonValue;
}

impl ToJsonValue for () {
    fn to_json_value(&self) -> JsonValue {
        JsonValue::Null
    }
}

impl ToJsonValue for u8 {
    fn to_json_value(&self) -> JsonValue {
        JsonValue::Number(Number::new(self.to_string()))
    }
}

impl ToJsonValue for u16 {
    fn to_json_value(&self) -> JsonValue {
        JsonValue::Number(Number::new(self.to_string()))
    }
}

impl ToJsonValue for u32 {
    fn to_json_value(&self) -> JsonValue {
        JsonValue::Number(Number::new(self.to_string()))
    }
}

impl ToJsonValue for u64 {
    fn to_json_value(&self) -> JsonValue {
        JsonValue::Number(Number::new(self.to_string()))
    }
}

impl ToJsonValue for u128 {
    fn to_json_value(&self) -> JsonValue {
        JsonValue::Number(Number::new(self.to_string()))
    }
}

impl ToJsonValue for usize {
    fn to_json_value(&self) -> JsonValue {
        JsonValue::Number(Number::new(self.to_string()))
    }
}

impl ToJsonValue for i8 {
    fn to_json_value(&self) -> JsonValue {
        JsonValue::Number(Number::new(self.to_string()))
    }
}

impl ToJsonValue for i16 {
    fn to_json_value(&self) -> JsonValue {
        JsonValue::Number(Number::new(self.to_string()))
    }
}

impl ToJsonValue for i32 {
    fn to_json_value(&self) -> JsonValue {
        JsonValue::Number(Number::new(self.to_string()))
    }
}

impl ToJsonValue for i64 {
    fn to_json_value(&self) -> JsonValue {
        JsonValue::Number(Number::new(self.to_string()))
    }
}

impl ToJsonValue for i128 {
    fn to_json_value(&self) -> JsonValue {
        JsonValue::Number(Number::new(self.to_string()))
    }
}

impl ToJsonValue for isize {
    fn to_json_value(&self) -> JsonValue {
        JsonValue::Number(Number::new(self.to_string()))
    }
}

impl ToJsonValue for f32 {
    fn to_json_value(&self) -> JsonValue {
        JsonValue::Number(Number::new(self.to_string()))
    }
}

impl ToJsonValue for f64 {
    fn to_json_value(&self) -> JsonValue {
        JsonValue::Number(Number::new(self.to_string()))
    }
}

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
