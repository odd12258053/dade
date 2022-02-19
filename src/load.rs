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
                _ if ('\u{30}'..='\u{39}').contains(&c) || c == '-' => {
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
                        '\u{30}' => val.push(cc),
                        // digit1-9
                        _ if ('\u{31}'..='\u{39}').contains(&cc) => {
                            val.push(cc);
                            // *DIGIT
                            loop {
                                match self.chars.peek() {
                                    Some(ccc) if ('\u{30}'..='\u{39}').contains(ccc) => {
                                        val.push(*ccc);
                                        self.chars.next();
                                    }
                                    _ => break,
                                }
                            }
                        }
                        _ => return Err(Error::new("extra data")),
                    }
                    // frac = decimal-point 1*DIGIT
                    // decimal-point
                    if let Some('.') = self.chars.peek() {
                        val.push(self.chars.next().unwrap());
                        // 1 DIGIT
                        match self.chars.next() {
                            Some(ccc) if ('\u{30}'..='\u{39}').contains(&ccc) => val.push(ccc),
                            _ => return Err(Error::new("extra data")),
                        }
                        // *DIGIT
                        loop {
                            match self.chars.peek() {
                                Some(ccc) if ('\u{30}'..='\u{39}').contains(ccc) => {
                                    val.push(*ccc);
                                    self.chars.next();
                                }
                                _ => break,
                            }
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
                            Some(ccc) if ('\u{30}'..='\u{39}').contains(&ccc) => val.push(ccc),
                            _ => return Err(Error::new("extra data")),
                        }
                        // *DIGIT
                        loop {
                            match self.chars.peek() {
                                Some(ccc) if ('\u{30}'..='\u{39}').contains(ccc) => {
                                    val.push(*ccc);
                                    self.chars.next();
                                }
                                _ => break,
                            }
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
                                Some('b') => val.push('\u{08}'),
                                Some('f') => val.push('\u{0C}'),
                                Some('n') => val.push('\n'),
                                Some('r') => val.push('\r'),
                                Some('t') => val.push('\t'),
                                Some('u') => {
                                    let mut buffer = Vec::new();
                                    loop {
                                        let mut code = 0u16;
                                        for i in [4096, 256, 16, 1] {
                                            code += match self.chars.next() {
                                                // 0..=9
                                                Some(ccc)
                                                    if ('\u{30}'..='\u{39}').contains(&ccc) =>
                                                {
                                                    ((ccc as u16) - 48) * i
                                                }
                                                // A..=F
                                                Some(ccc)
                                                    if ('\u{41}'..='\u{46}').contains(&ccc) =>
                                                {
                                                    ((ccc as u16) - 55) * i
                                                }
                                                // a..=f
                                                Some(ccc)
                                                    if ('\u{61}'..='\u{66}').contains(&ccc) =>
                                                {
                                                    ((ccc as u16) - 87) * i
                                                }
                                                _ => return Err(Error::new("extra data")),
                                            }
                                        }
                                        buffer.push(code);
                                        match String::from_utf16(&buffer) {
                                            Ok(s) => {
                                                val.push_str(s.as_str());
                                                break;
                                            }
                                            Err(err) => {
                                                if Some('\\') == self.chars.next()
                                                    && Some('u') == self.chars.next()
                                                {
                                                    continue;
                                                } else {
                                                    return Err(Error::new(
                                                        err.to_string().as_str(),
                                                    ));
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => return Err(Error::new("invalid control character")),
                            },
                            '"' => return Ok(JsonValue::String(val)),
                            _ if ('\u{0020}'..='\u{0021}').contains(&cc)
                                || ('\u{0023}'..='\u{005B}').contains(&cc)
                                || ('\u{005D}'..='\u{10FFFF}').contains(&cc) =>
                            {
                                val.push(cc)
                            }
                            _ => return Err(Error::new("invalid control character")),
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
                        if dict.contains_key(&key) {
                            return Err(Error::new("exists same key"));
                        }
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
