use crate::json::JsonValue;

pub struct JsonDumper {
    ensure_ascii: bool,
}

impl JsonDumper {
    pub fn new(ensure_ascii: bool) -> Self {
        Self { ensure_ascii }
    }
    pub fn escape(&self, val: &str) -> String {
        let mut duffer = String::from('"');
        for c in val.chars() {
            match c {
                '"' => duffer.push('\"'),
                _ if c.is_control() => {
                    duffer.push_str(c.escape_default().to_string().as_str());
                },
                _ if c.is_ascii() => {
                    duffer.push(c);
                }
                _ if self.ensure_ascii => {
                    let mut s = c.escape_unicode().to_string();
                    s.retain(|c| c != '{');
                    s.retain(|c| c != '}');
                    duffer.push_str(s.as_str());
                },
                _ => {
                    duffer.push(c);
                }
            }
        }
        duffer.push('"');
        duffer
    }
    pub fn dump(&self, value: &JsonValue) -> String {
        match value {
            JsonValue::Null => "null".to_string(),
            JsonValue::Bool(val) => val.to_string(),
            JsonValue::Number(val) => val.to_string(),
            JsonValue::String(val) => self.escape(val),
            JsonValue::Array(arr) => {
                let mut duffer = String::from('[');
                if !arr.is_empty() {
                    let mut iter = arr.iter();
                    duffer.push_str(self.dump(iter.next().unwrap()).as_str());
                    for val in iter {
                        duffer.push(',');
                        duffer.push_str(self.dump(val).as_str());
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
                    duffer.push_str(self.escape(key).as_str());
                    duffer.push(':');
                    duffer.push_str(self.dump(value).as_str());
                    for (key, value) in iter {
                        duffer.push(',');
                        duffer.push_str(self.escape(key).as_str());
                        duffer.push(':');
                        duffer.push_str(self.dump(value).as_str());
                    }
                }
                duffer.push('}');
                duffer
            }
        }
    }
}
