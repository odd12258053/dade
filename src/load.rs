use std::iter::Peekable;
use std::str::Chars;

use indexmap::IndexMap;

use crate::error::{Error, Result};
use crate::json::{JsonValue, Number};

pub struct JsonLoader<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> JsonLoader<'a> {
    pub fn new(json: &'a str) -> Self {
        Self {
            chars: json.chars().peekable(),
        }
    }

    pub fn load(&mut self) -> Result<JsonValue> {
        match self._load() {
            Ok(v) => {
                for c in self.chars.by_ref() {
                    match c {
                        ' ' | '\r' | '\t' | '\n' => continue,
                        _ => return Err(Error::new("extra data")),
                    }
                }
                Ok(v)
            }
            Err(err) => Err(err),
        }
    }

    fn _load(&mut self) -> Result<JsonValue> {
        while let Some(c) = self.chars.peek() {
            match c {
                ' ' | '\r' | '\t' | '\n' => {
                    self.chars.next();
                    continue;
                }
                // null
                'n' => {
                    if Some('n') == self.chars.next()
                        && Some('u') == self.chars.next()
                        && Some('l') == self.chars.next()
                        && Some('l') == self.chars.next()
                    {
                        return Ok(JsonValue::Null);
                    } else {
                        return Err(Error::new("extra data"));
                    }
                }
                // true
                't' => {
                    if Some('t') == self.chars.next()
                        && Some('r') == self.chars.next()
                        && Some('u') == self.chars.next()
                        && Some('e') == self.chars.next()
                    {
                        return Ok(JsonValue::Bool(true));
                    } else {
                        return Err(Error::new("extra data"));
                    }
                }
                // false
                'f' => {
                    if Some('f') == self.chars.next()
                        && Some('a') == self.chars.next()
                        && Some('l') == self.chars.next()
                        && Some('s') == self.chars.next()
                        && Some('e') == self.chars.next()
                    {
                        return Ok(JsonValue::Bool(false));
                    } else {
                        return Err(Error::new("extra data"));
                    }
                }
                // number = [ minus ] int [ frac ] [ exp ]
                // decimal-point = %x2E       ; .
                // digit1-9 = %x31-39         ; 1-9
                // e = %x65 / %x45            ; e E
                // exp = e [ minus / plus ] 1*DIGIT
                // frac = decimal-point 1*DIGIT
                // int = zero / ( digit1-9 *DIGIT )
                // minus = %x2D               ; -
                // plus = %x2B                ; +
                // zero = %x30                ; 0
                '-' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    let mut val = String::new();
                    // [ minus ]
                    if c == &'-' {
                        val.push(self.chars.next().unwrap());
                    }
                    // int = zero / ( digit1-9 *DIGIT )
                    match self.chars.peek() {
                        // zero
                        Some('0') => {
                            val.push(self.chars.next().unwrap());
                        }
                        // digit1-9
                        Some('1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9') => {
                            val.push(self.chars.next().unwrap());
                            // *DIGIT
                            while let Some(
                                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9',
                            ) = self.chars.peek()
                            {
                                val.push(self.chars.next().unwrap())
                            }
                        }
                        _ => return Err(Error::new("extra data")),
                    }
                    // frac = decimal-point 1*DIGIT
                    // decimal-point
                    if let Some('.') = self.chars.peek() {
                        val.push(self.chars.next().unwrap());
                        // *DIGIT
                        while let Some('0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9') =
                            self.chars.peek()
                        {
                            val.push(self.chars.next().unwrap())
                        }
                    }
                    // exp = e [ minus / plus ] 1*DIGIT
                    if let Some('e' | 'E') = self.chars.peek() {
                        val.push(self.chars.next().unwrap());
                        // [ minus / plus ]
                        if let Some('-' | '+') = self.chars.peek() {
                            val.push(self.chars.next().unwrap());
                        }
                        // *DIGIT
                        while let Some('0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9') =
                            self.chars.peek()
                        {
                            val.push(self.chars.next().unwrap())
                        }
                    }
                    return Ok(JsonValue::Number(Number::new(val)));
                }
                // string
                '"' => {
                    self.chars.next();
                    let mut val = String::new();
                    while let Some(cc) = self.chars.next() {
                        match cc {
                            '\\' => {
                                if let Some('"') = self.chars.peek() {
                                    val.push(cc);
                                    val.push(self.chars.next().unwrap());
                                }
                            }
                            '"' => break,
                            '\r' | '\t' | '\n' => {
                                return Err(Error::new("invalid control character"))
                            }
                            _ => val.push(cc),
                        }
                    }
                    return Ok(JsonValue::String(val));
                }
                // array
                '[' => {
                    self.chars.next();
                    let mut vec = Vec::new();
                    loop {
                        match self._load() {
                            Ok(val) => {
                                vec.push(val);
                                for cc in self.chars.by_ref() {
                                    match cc {
                                        ' ' | '\r' | '\t' | '\n' => continue,
                                        ',' => break,
                                        ']' => return Ok(JsonValue::Array(vec)),
                                        _ => return Err(Error::new("extra data")),
                                    }
                                }
                            }
                            Err(err) => {
                                if let Some(']') = self.chars.next() {
                                    return if vec.is_empty() {
                                        Ok(JsonValue::Array(vec))
                                    } else {
                                        Err(Error::new("expect ']'"))
                                    };
                                }
                                return Err(err);
                            }
                        }
                    }
                }
                // object
                '{' => {
                    self.chars.next();
                    let mut dict: IndexMap<String, JsonValue> = IndexMap::new();
                    loop {
                        match self._load() {
                            Ok(key_val) => {
                                let key = match key_val {
                                    JsonValue::String(key) => key,
                                    _ => return Err(Error::new("expect string")),
                                };
                                for cc in self.chars.by_ref() {
                                    match cc {
                                        ' ' | '\r' | '\t' | '\n' => continue,
                                        ':' => break,
                                        _ => return Err(Error::new("extra data")),
                                    }
                                }
                                match self._load() {
                                    Ok(val) => dict.insert(key, val),
                                    Err(err) => return Err(err),
                                };
                                for cc in self.chars.by_ref() {
                                    match cc {
                                        ' ' | '\r' | '\t' | '\n' => continue,
                                        ',' => break,
                                        '}' => return Ok(JsonValue::Object(dict)),
                                        _ => return Err(Error::new("extra data")),
                                    }
                                }
                            }
                            Err(err) => {
                                if let Some('}') = self.chars.next() {
                                    return if dict.is_empty() {
                                        Ok(JsonValue::Object(dict))
                                    } else {
                                        Err(Error::new("expect '}'"))
                                    };
                                }
                                return Err(err);
                            }
                        }
                    }
                }
                _ => break,
            }
        }
        Err(Error::new("expect value, found no data"))
    }
}
