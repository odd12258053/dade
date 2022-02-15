use crate::json::JsonValue;

pub struct JsonDumper {}

impl JsonDumper {
    pub fn escape(val: &str) -> String {
        let mut duffer = String::from('"');
        for c in val.chars() {
            match c {
                '"' => duffer.push('\"'),
                _ => duffer.push(c),
            }
        }
        duffer.push('"');
        duffer
    }
    pub fn dump(value: &JsonValue) -> String {
        match value {
            JsonValue::Null => "null".to_string(),
            JsonValue::Bool(val) => val.to_string(),
            JsonValue::Number(val) => val.to_string(),
            JsonValue::String(val) => JsonDumper::escape(val),
            JsonValue::Array(arr) => {
                let mut duffer = String::from('[');
                if !arr.is_empty() {
                    let mut iter = arr.iter();
                    duffer.push_str(JsonDumper::dump(iter.next().unwrap()).as_str());
                    for val in iter {
                        duffer.push(',');
                        duffer.push_str(JsonDumper::dump(val).as_str());
                    }
                }
                duffer.push(']');
                duffer
            }
            JsonValue::Object(dict) => {
                let mut duffer = String::from('{');
                if !dict.is_empty() {
                    let mut iter = dict.iter();
                    let (key, value) = iter.next().unwrap();
                    duffer.push_str(JsonDumper::escape(key).as_str());
                    duffer.push(':');
                    duffer.push_str(JsonDumper::dump(value).as_str());
                    for (key, value) in iter {
                        duffer.push(',');
                        duffer.push_str(JsonDumper::escape(key).as_str());
                        duffer.push(':');
                        duffer.push_str(JsonDumper::dump(value).as_str());
                    }
                }
                duffer.push('}');
                duffer
            }
        }
    }
}
