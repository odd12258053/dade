use crate::json::JsonValue;

const HEX_CODE: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

pub struct JsonDumper {
    ensure_ascii: bool,
    buffer: String,
}

impl JsonDumper {
    pub fn new(ensure_ascii: bool) -> Self {
        Self {
            ensure_ascii,
            buffer: String::new(),
        }
    }
    #[inline]
    pub fn escape(val: &str, buffer: &mut String, ensure_ascii: bool) {
        buffer.clear();
        buffer.push('"');
        for c in val.chars() {
            match c {
                '\u{0}' => buffer.push_str("\\u0000"),
                '\u{0022}' => buffer.push_str("\\\""),
                '\u{005C}' => buffer.push_str("\\\\"),
                // pass
                // '\u{002F}' => self.buffer.push_str("\\/"),
                '\u{0008}' => buffer.push_str("\\b"),
                '\u{000C}' => buffer.push_str("\\f"),
                '\u{000A}' => buffer.push_str("\\n"),
                '\u{000D}' => buffer.push_str("\\r"),
                '\u{0009}' => buffer.push_str("\\t"),
                '\u{0020}'..='\u{0021}' | '\u{0023}'..='\u{005B}' | '\u{005D}'..='\u{007E}' => {
                    buffer.push(c);
                }
                _ if !ensure_ascii && c.is_alphabetic() => {
                    buffer.push(c);
                }
                _ if ensure_ascii || !c.is_alphabetic() => {
                    if c.len_utf16() == 1 {
                        let code = c as u16;
                        buffer.push_str("\\u");
                        buffer.push(HEX_CODE[((code & 0xF000) >> 12) as usize]);
                        buffer.push(HEX_CODE[((code & 0x0F00) >> 8) as usize]);
                        buffer.push(HEX_CODE[((code & 0x00F0) >> 4) as usize]);
                        buffer.push(HEX_CODE[(code & 0x000F) as usize]);
                    } else {
                        let mut codes = [0; 2];
                        c.encode_utf16(&mut codes);
                        buffer.push_str("\\u");
                        buffer.push(HEX_CODE[((codes[0] & 0xF000) >> 12) as usize]);
                        buffer.push(HEX_CODE[((codes[0] & 0x0F00) >> 8) as usize]);
                        buffer.push(HEX_CODE[((codes[0] & 0x00F0) >> 4) as usize]);
                        buffer.push(HEX_CODE[(codes[0] & 0x000F) as usize]);
                        buffer.push_str("\\u");
                        buffer.push(HEX_CODE[((codes[1] & 0xF000) >> 12) as usize]);
                        buffer.push(HEX_CODE[((codes[1] & 0x0F00) >> 8) as usize]);
                        buffer.push(HEX_CODE[((codes[1] & 0x00F0) >> 4) as usize]);
                        buffer.push(HEX_CODE[(codes[1] & 0x000F) as usize]);
                    }
                }
                _ => buffer.push(c),
            }
        }
        buffer.push('"');
    }
    pub fn dump(&mut self, value: &JsonValue) -> String {
        match value {
            JsonValue::Null => "null".to_string(),
            JsonValue::Bool(val) => val.to_string(),
            JsonValue::Number(val) => val.to_string(),
            JsonValue::String(val) => {
                JsonDumper::escape(val, &mut self.buffer, self.ensure_ascii);
                self.buffer.to_string()
            }
            JsonValue::Array(arr) => {
                if arr.is_empty() {
                    return "[]".to_string();
                }
                let mut duffer = String::from('[');
                let mut iter = arr.iter();
                duffer.push_str(self.dump(iter.next().unwrap()).as_str());
                for val in iter {
                    duffer.push(',');
                    duffer.push_str(self.dump(val).as_str());
                }
                duffer.push(']');
                duffer
            }
            JsonValue::Object(dict) => {
                if dict.is_empty() {
                    return "{}".to_string();
                }
                let mut duffer = String::from('{');
                let mut iter = dict.iter();
                let (key, value) = iter.next().unwrap();
                duffer.push_str({
                    JsonDumper::escape(key, &mut self.buffer, self.ensure_ascii);
                    self.buffer.as_str()
                });
                duffer.push(':');
                duffer.push_str(self.dump(value).as_str());
                for (key, value) in iter {
                    duffer.push(',');
                    duffer.push_str({
                        JsonDumper::escape(key, &mut self.buffer, self.ensure_ascii);
                        self.buffer.as_str()
                    });
                    duffer.push(':');
                    duffer.push_str(self.dump(value).as_str());
                }
                duffer.push('}');
                duffer
            }
        }
    }
}
