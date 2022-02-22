use std::collections::BTreeMap;
use std::convert::From;
use std::iter::Iterator;
use std::iter::Peekable;

use crate::error::{Error, Result};
use crate::json::{JsonValue, Number};

pub struct JsonLoader<'a, T: Iterator<Item = &'a u8>> {
    bytes: Peekable<T>,
    buffer: Vec<u8>,
}

impl<'a> From<&'a str> for JsonLoader<'a, std::slice::Iter<'a, u8>> {
    fn from(json: &'a str) -> Self {
        Self {
            bytes: json.as_bytes().iter().peekable(),
            buffer: Vec::new(),
        }
    }
}

impl<'a, T: Iterator<Item = &'a u8>> JsonLoader<'a, T> {
    pub fn load(&mut self) -> Result<JsonValue> {
        let val = self._load()?;
        self.skip_control_char();
        if self.bytes.next().is_some() {
            return Err(Error::new_parse_err("extra data"));
        }
        Ok(val)
    }

    #[inline]
    fn skip_control_char(&mut self) {
        while self
            .bytes
            .next_if(|&b| *b == 0x20 || *b == 0x0d || *b == 0x09 || *b == 0x0a)
            .is_some()
        {}
    }

    fn get_number(&mut self, b: &u8) -> Result<JsonValue> {
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
        self.buffer.clear();
        macro_rules! one_digit {
            () => {
                match self.bytes.next() {
                    Some(b @ 0x30u8..=0x39) => self.buffer.push(*b),
                    _ => return Err(Error::new_parse_err("extra data")),
                }
            };
        }
        macro_rules! many_digit {
            () => {
                while let Some(b) = self.bytes.next_if(|&b| (0x30u8..=0x39).contains(b)) {
                    self.buffer.push(*b);
                }
            };
        }

        // [ minus ]
        let bb = if b == &0x2d {
            self.buffer.push(0x2d);
            match self.bytes.next() {
                Some(bb) => bb,
                None => return Err(Error::new_parse_err("extra data")),
            }
        } else {
            b
        };
        // int = zero / ( digit1-9 *DIGIT )
        match bb {
            // zero
            0x30 => self.buffer.push(*bb),
            // digit1-9
            0x31..=0x39 => {
                self.buffer.push(*bb);
                // *DIGIT
                many_digit!();
            }
            _ => return Err(Error::new_parse_err("extra data")),
        }
        // frac = decimal-point 1*DIGIT
        // decimal-point
        if self.bytes.next_if_eq(&&0x2eu8).is_some() {
            self.buffer.push(0x2e);
            // 1 DIGIT
            one_digit!();
            // *DIGIT
            many_digit!();
        }
        // exp = e [ minus / plus ] 1*DIGIT
        if let Some(bb) = self.bytes.next_if(|&b| *b == 0x65 || *b == 0x45) {
            self.buffer.push(*bb);
            // [ minus / plus ]
            if let Some(bbb) = self.bytes.next_if(|&b| *b == 0x2d || *b == 0x2b) {
                self.buffer.push(*bbb);
            }
            // 1 DIGIT
            one_digit!();
            // *DIGIT
            many_digit!();
        }
        unsafe {
            Ok(JsonValue::Number(Number::new(String::from_utf8_unchecked(
                Vec::from(self.buffer.as_slice()),
            ))))
        }
    }

    #[inline]
    fn handle_escaped_unicode(&mut self) -> Result<()> {
        // ref: https://www.unicode.org/versions/Unicode14.0.0/ch03.pdf
        macro_rules! to_num {
            () => {
                match self.bytes.next() {
                    // 0..=9
                    Some(bbb @ 0x30u8..=0x39) => ((*bbb as u16) - 48),
                    // A..=F
                    Some(bbb @ 0x41..=0x46) => ((*bbb as u16) - 55),
                    // a..=f
                    Some(bbb @ 0x61..=0x66) => ((*bbb as u16) - 87),
                    _ => return Err(Error::new_parse_err("extra data")),
                }
            };
        }

        let code = (to_num!() << 12) + (to_num!() << 8) + (to_num!() << 4) + to_num!();
        if !(0xD800..=0xDFFF).contains(&code) {
            if code < 0x80 {
                self.buffer.push((code & 0x00FF) as u8)
            } else if code < 0x800 {
                self.buffer.extend_from_slice(&[
                    (0b11000000 | ((code & 0b0000011111000000) >> 6)) as u8,
                    (0b10000000 | (code & 0b0000000000111111)) as u8,
                ]);
            } else {
                self.buffer.extend_from_slice(&[
                    (0b11100000 | ((code & 0b1111000000000000) >> 12)) as u8,
                    (0b10000000 | ((code & 0b0000111111000000) >> 6)) as u8,
                    (0b10000000 | (code & 0b0000000000111111)) as u8,
                ]);
            }
        } else {
            if code > 0xDBFF {
                return Err(Error::new_parse_err("no surrogate key"));
            }
            if !(Some(&0x5c) == self.bytes.next() && Some(&0x75) == self.bytes.next()) {
                return Err(Error::new_parse_err("extra data"));
            }
            let code2 = (to_num!() << 12) + (to_num!() << 8) + (to_num!() << 4) + to_num!();
            if !(0xDC00..=0xDFFF).contains(&code2) {
                return Err(Error::new_parse_err("no surrogate key"));
            }
            let u = ((code & 0b0000001111000000) >> 6) + 1;
            let x = ((code & 0b0000000000111111) << 10) + (code2 & 0b0000001111111111);
            self.buffer.extend_from_slice(&[
                (0b11110000 + (u >> 2)) as u8,
                (0b10000000 + ((0b00000011 & u) << 4) + ((0b1111000000000000 & x) >> 12)) as u8,
                (0b10000000 + ((0b0000111111000000 & x) >> 6)) as u8,
                (0b10000000 + (0b0000000000111111 & x)) as u8,
            ]);
        }
        Ok(())
    }

    #[inline]
    fn handle_escaped_str(&mut self) -> Result<()> {
        match self.bytes.next() {
            Some(0x22) => self.buffer.push(0x22),
            Some(0x5c) => self.buffer.push(0x5c),
            Some(0x2f) => self.buffer.push(0x2f),
            Some(0x62) => self.buffer.push(0x08),
            Some(0x66) => self.buffer.push(0x0c),
            Some(0x6e) => self.buffer.push(0x0a),
            Some(0x72) => self.buffer.push(0x0d),
            Some(0x74) => self.buffer.push(0x09),
            Some(0x75) => self.handle_escaped_unicode()?,
            _ => return Err(Error::new_parse_err("invalid control character")),
        }
        Ok(())
    }

    fn _get_string(&mut self) -> Result<String> {
        // string = quotation-mark *char quotation-mark
        //
        // char = unescaped /
        //     escape (
        //         %x22 /          ; "    quotation mark  U+0022
        //         %x5C /          ; \    reverse solidus U+005C
        //         %x2F /          ; /    solidus         U+002F
        //         %x62 /          ; b    backspace       U+0008
        //         %x66 /          ; f    form feed       U+000C
        //         %x6E /          ; n    line feed       U+000A
        //         %x72 /          ; r    carriage return U+000D
        //         %x74 /          ; t    tab             U+0009
        //         %x75 4HEXDIG )  ; uXXXX                U+XXXX
        //
        // escape = %x5C              ; \
        //
        // quotation-mark = %x22      ; "
        //
        // unescaped = %x20-21 / %x23-5B / %x5D-10FFFF
        self.buffer.clear();
        let mut buf = [0; 4];

        macro_rules! handle_unicode {
            ($i: literal, $range: pat_param) => {
                match self.bytes.next() {
                    Some(bbb @ $range) => buf[$i] = *bbb,
                    _ => return Err(Error::new_parse_err("invalid utf-8 character")),
                }
            };
        }

        loop {
            match self.bytes.next() {
                Some(0x22) => unsafe {
                    return Ok(String::from_utf8_unchecked(Vec::from(
                        self.buffer.as_slice(),
                    )));
                },
                Some(0x5c) => self.handle_escaped_str()?,
                // ref: https://www.unicode.org/versions/Unicode14.0.0/ch03.pdf
                Some(bb @ (0x20..=0x21 | 0x23..=0x5b | 0x5d..=0x7f)) => {
                    self.buffer.push(*bb);
                }
                Some(bb @ 0xc2..=0xdf) => {
                    buf[0] = *bb;
                    handle_unicode!(1, 0x80..=0xbf);
                    self.buffer.extend_from_slice(&buf[..2])
                }
                Some(0xe0) => {
                    buf[0] = 0xe0;
                    handle_unicode!(1, 0xa0..=0xbf);
                    handle_unicode!(2, 0x80..=0xbf);
                    self.buffer.extend_from_slice(&buf[..3])
                }
                Some(bb @ 0xe1..=0xec) => {
                    buf[0] = *bb;
                    handle_unicode!(1, 0x80..=0xbf);
                    handle_unicode!(2, 0x80..=0xbf);
                    self.buffer.extend_from_slice(&buf[..3])
                }
                Some(0xed) => {
                    buf[0] = 0xed;
                    handle_unicode!(1, 0x80..=0x9f);
                    handle_unicode!(2, 0x80..=0xbf);
                    self.buffer.extend_from_slice(&buf[..3])
                }
                Some(bb @ 0xee..=0xef) => {
                    buf[0] = *bb;
                    handle_unicode!(1, 0x80..=0xbf);
                    handle_unicode!(2, 0x80..=0xbf);
                    self.buffer.extend_from_slice(&buf[..3])
                }
                Some(0xf0) => {
                    buf[0] = 0xf0;
                    handle_unicode!(1, 0x90..=0xbf);
                    handle_unicode!(2, 0x80..=0xbf);
                    handle_unicode!(3, 0x80..=0xbf);
                    self.buffer.extend_from_slice(&buf)
                }
                Some(bb @ 0xf1..=0xf3) => {
                    buf[0] = *bb;
                    handle_unicode!(1, 0x80..=0xbf);
                    handle_unicode!(2, 0x80..=0xbf);
                    handle_unicode!(3, 0x80..=0xbf);
                    self.buffer.extend_from_slice(&buf)
                }
                Some(0xf4) => {
                    buf[0] = 0xf4;
                    handle_unicode!(1, 0x80..=0xbf);
                    handle_unicode!(2, 0x80..=0xbf);
                    handle_unicode!(3, 0x80..=0xbf);
                    self.buffer.extend_from_slice(&buf)
                }
                None => return Err(Error::new_parse_err("unterminated string")),
                _ => return Err(Error::new_parse_err("invalid control character")),
            }
        }
    }

    fn get_string(&mut self) -> Result<JsonValue> {
        Ok(JsonValue::String(self._get_string()?))
    }

    fn get_null(&mut self) -> Result<JsonValue> {
        if self
            .bytes
            .next_if_eq(&&0x75u8)
            .and(self.bytes.next_if_eq(&&0x6cu8))
            .and(self.bytes.next_if_eq(&&0x6cu8))
            .is_some()
        {
            Ok(JsonValue::Null)
        } else {
            Err(Error::new_parse_err("extra data"))
        }
    }

    fn get_true(&mut self) -> Result<JsonValue> {
        if self
            .bytes
            .next_if_eq(&&0x72u8)
            .and(self.bytes.next_if_eq(&&0x75u8))
            .and(self.bytes.next_if_eq(&&0x65u8))
            .is_some()
        {
            Ok(JsonValue::Bool(true))
        } else {
            Err(Error::new_parse_err("extra data"))
        }
    }

    fn get_false(&mut self) -> Result<JsonValue> {
        if self
            .bytes
            .next_if_eq(&&0x61u8)
            .and(self.bytes.next_if_eq(&&0x6cu8))
            .and(self.bytes.next_if_eq(&&0x73u8))
            .and(self.bytes.next_if_eq(&&0x65u8))
            .is_some()
        {
            Ok(JsonValue::Bool(false))
        } else {
            Err(Error::new_parse_err("extra data"))
        }
    }

    fn get_array(&mut self) -> Result<JsonValue> {
        self.skip_control_char();
        if self.bytes.next_if_eq(&&0x5du8).is_some() {
            return Ok(JsonValue::Array(Vec::new()));
        }
        let mut vec = Vec::new();
        loop {
            vec.push(self._load()?);
            self.skip_control_char();
            match self.bytes.next() {
                Some(0x2c) => {}
                Some(0x5d) => return Ok(JsonValue::Array(vec)),
                _ => return Err(Error::new_parse_err("extra data")),
            }
        }
    }

    fn get_object(&mut self) -> Result<JsonValue> {
        self.skip_control_char();
        let mut dict = BTreeMap::new();
        if self.bytes.next_if_eq(&&0x7du8).is_some() {
            return Ok(JsonValue::Object(dict));
        }
        loop {
            self.skip_control_char();
            if self.bytes.next_if_eq(&&0x22).is_some() {
                let key = self._get_string()?;
                self.skip_control_char();
                if self.bytes.next_if_eq(&&0x3au8).is_none() {
                    return Err(Error::new_parse_err("extra data"));
                }
                if dict.insert(key, self._load()?).is_some() {
                    return Err(Error::new_parse_err("exists same key"));
                }
                self.skip_control_char();
                match self.bytes.next() {
                    Some(0x2c) => {}
                    Some(0x7d) => return Ok(JsonValue::Object(dict)),
                    _ => return Err(Error::new_parse_err("extra data")),
                }
            } else {
                return Err(Error::new_parse_err("expect string"));
            }
        }
    }

    #[inline]
    fn _load(&mut self) -> Result<JsonValue> {
        self.skip_control_char();
        match self.bytes.next() {
            // null
            Some(&0x6e) => self.get_null(),
            // true
            Some(&0x74) => self.get_true(),
            // false
            Some(&0x66) => self.get_false(),
            // number
            Some(b @ (0x30..=0x39 | 0x2d)) => self.get_number(b),
            // string
            Some(&0x22) => self.get_string(),
            // array
            Some(&0x5b) => self.get_array(),
            // object
            Some(&0x7b) => self.get_object(),
            _ => Err(Error::new_parse_err("expect value, found no data")),
        }
    }
}
