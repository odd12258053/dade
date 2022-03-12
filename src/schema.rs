use std::collections::BTreeMap;

use crate::json::JsonValue;

/// A trait defines the format to define the schema for a model or a field.
pub trait RegisterSchema {
    fn register_schema(defs: &mut BTreeMap<String, JsonValue>) -> JsonValue;
}

impl RegisterSchema for () {
    fn register_schema(_defs: &mut BTreeMap<String, JsonValue>) -> JsonValue {
        JsonValue::Object(BTreeMap::from([(
            "type".to_string(),
            JsonValue::String("null".to_string()),
        )]))
    }
}

impl RegisterSchema for String {
    fn register_schema(_defs: &mut BTreeMap<String, JsonValue>) -> JsonValue {
        JsonValue::Object(BTreeMap::from([(
            "type".to_string(),
            JsonValue::String("string".to_string()),
        )]))
    }
}

impl<T: RegisterSchema> RegisterSchema for Vec<T> {
    fn register_schema(defs: &mut BTreeMap<String, JsonValue>) -> JsonValue {
        JsonValue::Object(BTreeMap::from([
            ("type".to_string(), JsonValue::String("array".to_string())),
            (
                "items".to_string(),
                <T as RegisterSchema>::register_schema(defs),
            ),
        ]))
    }
}

impl<T: RegisterSchema> RegisterSchema for Option<T> {
    fn register_schema(defs: &mut BTreeMap<String, JsonValue>) -> JsonValue {
        JsonValue::Object(BTreeMap::from([(
            "anyOf".to_string(),
            JsonValue::Array(Vec::from([
                JsonValue::Object(BTreeMap::from([(
                    "type".to_string(),
                    JsonValue::String("null".to_string()),
                )])),
                <T as RegisterSchema>::register_schema(defs),
            ])),
        )]))
    }
}

impl<T: RegisterSchema> RegisterSchema for Box<T> {
    fn register_schema(defs: &mut BTreeMap<String, JsonValue>) -> JsonValue {
        <T as RegisterSchema>::register_schema(defs)
    }
}

impl RegisterSchema for bool {
    fn register_schema(_defs: &mut BTreeMap<String, JsonValue>) -> JsonValue {
        JsonValue::Object(BTreeMap::from([(
            "type".to_string(),
            JsonValue::String("boolean".to_string()),
        )]))
    }
}

macro_rules! int_schema {
    ( $( $i:ident ),* ) => {
        $(
            impl RegisterSchema for $i {
                fn register_schema(_defs: &mut BTreeMap<String, JsonValue>) -> JsonValue {
                    JsonValue::Object(BTreeMap::from([(
                        "type".to_string(),
                        JsonValue::String("integer".to_string()),
                    )]))
                }
            }
        )*
    };
}

int_schema!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

macro_rules! float_schema {
    ( $( $i:ident ),* ) => {
        $(
            impl RegisterSchema for $i {
                fn register_schema(_defs: &mut BTreeMap<String, JsonValue>) -> JsonValue {
                    JsonValue::Object(BTreeMap::from([(
                        "type".to_string(),
                        JsonValue::String("number".to_string()),
                    )]))
                }
            }
        )*
    };
}

float_schema!(f32, f64);
