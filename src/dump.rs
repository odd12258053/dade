use crate::json::JsonValue;

pub struct JsonDumper {
    ensure_ascii: bool,
}

impl JsonDumper {
    pub fn new(ensure_ascii: bool) -> Self {
        Self { ensure_ascii }
    }
    pub fn escape(&self, val: &str) -> String {
        let mut buffer = String::with_capacity(val.len());
        buffer.push('"');
        for c in val.chars() {
            match c {
                '\u{0}' => buffer.push_str("\\u0000"),
                '\u{0022}' => buffer.push_str("\\\""),
                '\u{005C}' => buffer.push_str("\\\\"),
                // pass
                // '\u{002F}' => buffer.push_str("\\/"),
                '\u{0008}' => buffer.push_str("\\b"),
                '\u{000C}' => buffer.push_str("\\f"),
                '\u{000A}' => buffer.push_str("\\n"),
                '\u{000D}' => buffer.push_str("\\r"),
                '\u{0009}' => buffer.push_str("\\t"),
                _ if ('\u{0020}'..='\u{0021}').contains(&c)
                    || ('\u{0023}'..='\u{005B}').contains(&c)
                    || ('\u{005D}'..='\u{007E}').contains(&c) =>
                {
                    buffer.push(c);
                }
                _ if !self.ensure_ascii && c.is_alphabetic() => {
                    buffer.push(c);
                }
                _ if self.ensure_ascii || !c.is_alphabetic() => {
                    let mut codes = [0; 2];
                    c.encode_utf16(&mut codes);
                    for code in &codes[..c.len_utf16()] {
                        if *code > 0 {
                            let mut codes = String::with_capacity(6);
                            codes.push('\\');
                            codes.push('u');
                            for (i, pos) in [(12, 0xF000), (8, 0x0F00), (4, 0x00F0), (0, 0x000F)] {
                                match (code & pos) >> i {
                                    0 => codes.push('0'),
                                    1 => codes.push('1'),
                                    2 => codes.push('2'),
                                    3 => codes.push('3'),
                                    4 => codes.push('4'),
                                    5 => codes.push('5'),
                                    6 => codes.push('6'),
                                    7 => codes.push('7'),
                                    8 => codes.push('8'),
                                    9 => codes.push('9'),
                                    10 => codes.push('a'),
                                    11 => codes.push('b'),
                                    12 => codes.push('c'),
                                    13 => codes.push('d'),
                                    14 => codes.push('e'),
                                    15 => codes.push('f'),
                                    _ => panic!("unexpect value"),
                                }
                            }
                            buffer.push_str(codes.as_str());
                        }
                    }
                }
                _ => buffer.push(c),
            }
        }
        buffer.push('"');
        buffer
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
