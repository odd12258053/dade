use std::iter::Peekable;
use std::str::Chars;

use indexmap::IndexMap;

use crate::error::{Error, Result};
use crate::json::{JsonValue, Number};

const BUFFER_SIZE_STRING: usize = 256;
const BUFFER_SIZE_NUMBER: usize = 32;
const BUFFER_SIZE_ARRAY: usize = 8;
const BUFFER_SIZE_OBJECT: usize = 8;

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
        let val = self._load()?;
        for c in self.chars.by_ref() {
            match c {
                ' ' | '\r' | '\t' | '\n' => continue,
                _ => return Err(Error::new("extra data")),
            }
        }
        Ok(val)
    }

    fn _load(&mut self) -> Result<JsonValue> {
        while let Some(c) = self.chars.next() {
            match c {
                ' ' | '\r' | '\t' | '\n' => continue,
                // null
                'n' => {
                    if Some('u') == self.chars.next()
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
                    if Some('r') == self.chars.next()
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
                    if Some('a') == self.chars.next()
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
                    let mut val = String::with_capacity(BUFFER_SIZE_NUMBER);
                    // [ minus ]
                    let cc = if c == '-' {
                        val.push(c);
                        match self.chars.next() {
                            Some(cc) => cc,
                            None => return Err(Error::new("extra data")),
                        }
                    } else {
                        c
                    };
                    // int = zero / ( digit1-9 *DIGIT )
                    match cc {
                        // zero
                        '0' => {
                            val.push(cc);
                        }
                        // digit1-9
                        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                            val.push(cc);
                            // *DIGIT
                            while let Some(
                                ccc @ ('0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'),
                            ) = self.chars.peek()
                            {
                                val.push(*ccc);
                                self.chars.next();
                            }
                        }
                        _ => return Err(Error::new("extra data")),
                    }
                    // frac = decimal-point 1*DIGIT
                    // decimal-point
                    if let Some(cc @ '.') = self.chars.peek() {
                        val.push(*cc);
                        self.chars.next();
                        // *DIGIT
                        while let Some(
                            ccc @ ('0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'),
                        ) = self.chars.peek()
                        {
                            val.push(*ccc);
                            self.chars.next();
                        }
                    }
                    // exp = e [ minus / plus ] 1*DIGIT
                    if let Some(cc @ ('e' | 'E')) = self.chars.peek() {
                        val.push(*cc);
                        self.chars.next();
                        // [ minus / plus ]
                        if let Some(ccc @ ('-' | '+')) = self.chars.peek() {
                            val.push(*ccc);
                            self.chars.next();
                        }
                        // 1 DIGIT
                        match self.chars.next() {
                            Some(
                                ccc @ ('0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'),
                            ) => val.push(ccc),
                            _ => return Err(Error::new("extra data")),
                        }
                        // *DIGIT
                        while let Some(
                            cc @ ('0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'),
                        ) = self.chars.peek()
                        {
                            val.push(*cc);
                            self.chars.next();
                        }
                    }
                    return Ok(JsonValue::Number(Number::new(val)));
                }
                // string
                '"' => {
                    let mut val = String::with_capacity(BUFFER_SIZE_STRING);
                    while let Some(cc) = self.chars.next() {
                        match cc {
                            '\\' => match self.chars.next() {
                                Some('"') => val.push('"'),
                                Some('\\') => val.push('\\'),
                                Some('/') => val.push('/'),
                                Some('b') => val.push(char::from_u32(0x08).unwrap()),
                                Some('f') => val.push(char::from_u32(0x0C).unwrap()),
                                Some('n') => val.push('\n'),
                                Some('r') => val.push('\r'),
                                Some('t') => val.push('\t'),
                                Some('u') => {
                                    match char::from_u32({
                                        let mut code = 0;
                                        for i in [4096, 256, 16, 1] {
                                            match self.chars.next() {
                                                Some('0') => continue,
                                                Some('1') => code += i,
                                                Some('2') => code += i * 2,
                                                Some('3') => code += i * 3,
                                                Some('4') => code += i * 4,
                                                Some('5') => code += i * 5,
                                                Some('6') => code += i * 6,
                                                Some('7') => code += i * 7,
                                                Some('8') => code += i * 8,
                                                Some('9') => code += i * 9,
                                                Some('a' | 'A') => code += i * 10,
                                                Some('b' | 'B') => code += i * 11,
                                                Some('c' | 'C') => code += i * 12,
                                                Some('d' | 'D') => code += i * 13,
                                                Some('e' | 'E') => code += i * 14,
                                                Some('f' | 'F') => code += i * 15,
                                                _ => return Err(Error::new("extra data")),
                                            }
                                        }
                                        code
                                    }) {
                                        Some(ccc) => val.push(ccc),
                                        None => return Err(Error::new("extra data")),
                                    }
                                }
                                _ => return Err(Error::new("invalid control character")),
                            },
                            '"' => return Ok(JsonValue::String(val)),
                            '\r' | '\t' | '\n' => {
                                return Err(Error::new("invalid control character"))
                            }
                            _ => val.push(cc),
                        }
                    }
                    return Err(Error::new("unterminated string"));
                }
                // array
                '[' => {
                    let mut vec = Vec::with_capacity(BUFFER_SIZE_ARRAY);
                    loop {
                        match self.chars.peek() {
                            Some(' ' | '\r' | '\t' | '\n') => {
                                self.chars.next();
                            }
                            Some(']') => {
                                self.chars.next();
                                return Ok(JsonValue::Array(vec));
                            }
                            _ => break,
                        }
                    }
                    loop {
                        vec.push(self._load()?);
                        for cc in self.chars.by_ref() {
                            match cc {
                                ' ' | '\r' | '\t' | '\n' => continue,
                                ',' => break,
                                ']' => return Ok(JsonValue::Array(vec)),
                                _ => return Err(Error::new("extra data")),
                            }
                        }
                    }
                }
                // object
                '{' => {
                    let mut dict: IndexMap<String, JsonValue> =
                        IndexMap::with_capacity(BUFFER_SIZE_OBJECT);
                    loop {
                        match self.chars.peek() {
                            Some(' ' | '\r' | '\t' | '\n') => {
                                self.chars.next();
                            }
                            Some('}') => {
                                self.chars.next();
                                return Ok(JsonValue::Object(dict));
                            }
                            _ => break,
                        }
                    }

                    loop {
                        let key = match self._load()? {
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
                        dict.insert(key, self._load()?);
                        for cc in self.chars.by_ref() {
                            match cc {
                                ' ' | '\r' | '\t' | '\n' => continue,
                                ',' => break,
                                '}' => return Ok(JsonValue::Object(dict)),
                                _ => return Err(Error::new("extra data")),
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
