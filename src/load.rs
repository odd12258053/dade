use std::collections::BTreeMap;
use std::convert::From;

use crate::error::{Error, Result};
use crate::json::{JsonValue, Number};
use crate::stream::{SliceBytes, Stream};

static JSON_CTR: [bool; 256] = {
    const CT: bool = true;
    const __: bool = false;
    [
        //   1,  2,  3,  4,  5,  6,  7,  8,  9,  A,  B,  C,  D,  E,  F,
        __, __, __, __, __, __, __, __, __, CT, CT, __, __, CT, __, __, // 0
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 1
        CT, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 2
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 3
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 4
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 5
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 6
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 7
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 8
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 9
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // A
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // B
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // C
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // D
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // E
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // F
    ]
};

pub struct JsonLoader<T, S: Stream<T>> {
    bytes: S,
    buffer: Vec<T>,
}

impl<'a> From<&'a str> for JsonLoader<u8, SliceBytes<'a>> {
    fn from(json: &'a str) -> Self {
        Self {
            bytes: SliceBytes::new(json.as_bytes()),
            buffer: Vec::new(),
        }
    }
}

impl<'a> From<&'a [u8]> for JsonLoader<u8, SliceBytes<'a>> {
    fn from(bytes: &'a [u8]) -> Self {
        Self {
            bytes: SliceBytes::new(bytes),
            buffer: Vec::new(),
        }
    }
}

impl<S: Stream<u8>> JsonLoader<u8, S> {
    pub fn load(&mut self) -> Result<JsonValue> {
        let val = self._load()?;
        self.skip_control_char();
        if !self.bytes.finish() {
            return Err(Error::parse_err("extra data"));
        }
        Ok(val)
    }

    #[inline]
    fn skip_control_char(&mut self) {
        self.bytes.while_if(|&b| JSON_CTR[b as usize]);
    }

    #[inline]
    fn get_number(&mut self) -> Result<JsonValue> {
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
        self.bytes.anchor();
        // [ minus ]
        self.bytes.next_if(|&b| b == 0x2d);
        // int = zero / ( digit1-9 *DIGIT )
        match self.bytes.next() {
            // digit1-9
            Some(0x31..=0x39) => {
                // *DIGIT
                self.bytes.while_if(|b| (0x30..=0x39).contains(b));
            }
            // zero
            Some(0x30) => {}
            _ => return Err(Error::parse_err("extra data")),
        }
        // frac = decimal-point 1*DIGIT
        // decimal-point
        if self.bytes.next_if(|&b| b == 0x2e) {
            // 1 DIGIT
            if !self.bytes.next_if(|b| (0x30..=0x39).contains(b)) {
                return Err(Error::parse_err("extra data"));
            }
            // *DIGIT
            self.bytes.while_if(|b| (0x30..=0x39).contains(b));
        }
        // exp = e [ minus / plus ] 1*DIGIT
        if self.bytes.next_if(|&b| b == 0x65 || b == 0x45) {
            // [ minus / plus ]
            self.bytes.next_if(|&b| b == 0x2d || b == 0x2b);
            // 1 DIGIT
            if !self.bytes.next_if(|b| (0x30..=0x39).contains(b)) {
                return Err(Error::parse_err("extra data"));
            }
            // *DIGIT
            self.bytes.while_if(|b| (0x30..=0x39).contains(b))
        }
        unsafe {
            Ok(JsonValue::Number(Number::new(String::from_utf8_unchecked(
                Vec::from(self.bytes.read()),
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
                    _ => return Err(Error::parse_err("extra data")),
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
                return Err(Error::parse_err("no surrogate key"));
            }
            if !(Some(&0x5c) == self.bytes.next() && Some(&0x75) == self.bytes.next()) {
                return Err(Error::parse_err("extra data"));
            }
            let code2 = (to_num!() << 12) + (to_num!() << 8) + (to_num!() << 4) + to_num!();
            if !(0xDC00..=0xDFFF).contains(&code2) {
                return Err(Error::parse_err("no surrogate key"));
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
            Some(0x75) => self.handle_escaped_unicode()?,
            Some(0x22) => self.buffer.push(0x22),
            Some(0x5c) => self.buffer.push(0x5c),
            Some(0x2f) => self.buffer.push(0x2f),
            Some(0x62) => self.buffer.push(0x08),
            Some(0x66) => self.buffer.push(0x0c),
            Some(0x6e) => self.buffer.push(0x0a),
            Some(0x72) => self.buffer.push(0x0d),
            Some(0x74) => self.buffer.push(0x09),
            _ => return Err(Error::parse_err("invalid control character")),
        }
        Ok(())
    }

    #[inline]
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
        self.bytes.anchor();
        self.buffer.clear();

        macro_rules! handle_unicode {
            ($range: expr) => {
                if !self.bytes.next_if(|b| $range.contains(b)) {
                    return Err(Error::parse_err("invalid utf-8 character"));
                }
            };
        }

        loop {
            match self.bytes.peek() {
                // ref: https://www.unicode.org/versions/Unicode14.0.0/ch03.pdf
                Some(0x20..=0x21 | 0x23..=0x5b | 0x5d..=0x7f) => {
                    self.bytes.skip();
                }
                Some((0xe1..=0xec)) => {
                    self.bytes.skip();
                    handle_unicode!(0x80..=0xbf);
                    handle_unicode!(0x80..=0xbf);
                }
                Some(0x22) => unsafe {
                    self.buffer.extend_from_slice(self.bytes.read());
                    self.bytes.skip();
                    return Ok(String::from_utf8_unchecked(Vec::from(
                        self.buffer.as_slice(),
                    )));
                },
                Some(0x5c) => {
                    self.buffer.extend_from_slice(self.bytes.read());
                    self.bytes.skip();
                    self.handle_escaped_str()?;
                    self.bytes.anchor();
                }
                Some((0xc2..=0xdf)) => {
                    self.bytes.skip();
                    handle_unicode!(0x80..=0xbf);
                }
                Some(0xe0) => {
                    self.bytes.skip();
                    handle_unicode!(0xa0..=0xbf);
                    handle_unicode!(0x80..=0xbf);
                }
                Some(0xed) => {
                    self.bytes.skip();
                    handle_unicode!(0x80..=0x9f);
                    handle_unicode!(0x80..=0xbf);
                }
                Some((0xee..=0xef)) => {
                    self.bytes.skip();
                    handle_unicode!(0x80..=0xbf);
                    handle_unicode!(0x80..=0xbf);
                }
                Some(0xf0) => {
                    self.bytes.skip();
                    handle_unicode!(0x90..=0xbf);
                    handle_unicode!(0x80..=0xbf);
                    handle_unicode!(0x80..=0xbf);
                }
                Some((0xf1..=0xf3)) => {
                    self.bytes.skip();
                    handle_unicode!(0x80..=0xbf);
                    handle_unicode!(0x80..=0xbf);
                    handle_unicode!(0x80..=0xbf);
                }
                Some(0xf4) => {
                    self.bytes.skip();
                    handle_unicode!(0x80..=0x8f);
                    handle_unicode!(0x80..=0xbf);
                    handle_unicode!(0x80..=0xbf);
                }
                None => return Err(Error::parse_err("unterminated string")),
                _ => return Err(Error::parse_err("invalid control character")),
            }
        }
    }

    #[inline]
    fn get_string(&mut self) -> Result<JsonValue> {
        Ok(JsonValue::String(self._get_string()?))
    }

    #[inline]
    fn get_null(&mut self) -> Result<JsonValue> {
        if self.bytes.expect(b"null") {
            Ok(JsonValue::Null)
        } else {
            Err(Error::parse_err("extra data"))
        }
    }

    #[inline]
    fn get_true(&mut self) -> Result<JsonValue> {
        if self.bytes.expect(b"true") {
            Ok(JsonValue::Bool(true))
        } else {
            Err(Error::parse_err("extra data"))
        }
    }

    #[inline]
    fn get_false(&mut self) -> Result<JsonValue> {
        if self.bytes.expect(b"false") {
            Ok(JsonValue::Bool(false))
        } else {
            Err(Error::parse_err("extra data"))
        }
    }

    #[inline]
    fn get_array(&mut self) -> Result<JsonValue> {
        self.skip_control_char();
        if self.bytes.next_if(|&b| b == 0x5d) {
            return Ok(JsonValue::Array(Vec::new()));
        }
        let mut vec = Vec::new();
        loop {
            vec.push(self._load()?);
            self.skip_control_char();
            match self.bytes.next() {
                Some(0x2c) => {}
                Some(0x5d) => return Ok(JsonValue::Array(vec)),
                _ => return Err(Error::parse_err("extra data")),
            }
        }
    }

    #[inline]
    fn get_object(&mut self) -> Result<JsonValue> {
        self.skip_control_char();
        if self.bytes.next_if(|&b| b == 0x7d) {
            return Ok(JsonValue::Object(BTreeMap::new()));
        }
        let mut dict = BTreeMap::new();

        self.skip_control_char();
        while Some(&0x22) == self.bytes.next() {
            let key = self._get_string()?;
            self.skip_control_char();
            if !self.bytes.next_if(|&b| b == 0x3a) {
                return Err(Error::parse_err("extra data"));
            }
            if dict.insert(key, self._load()?).is_some() {
                return Err(Error::parse_err("exists same key"));
            }
            self.skip_control_char();
            match self.bytes.next() {
                Some(0x2c) => {}
                Some(0x7d) => return Ok(JsonValue::Object(dict)),
                _ => return Err(Error::parse_err("extra data")),
            }
            self.skip_control_char();
        }
        Err(Error::parse_err("expect string"))
    }

    #[inline]
    fn _load(&mut self) -> Result<JsonValue> {
        self.skip_control_char();
        match self.bytes.peek() {
            // string
            Some(0x22) => {
                self.bytes.skip();
                self.get_string()
            }
            // number
            Some(0x30..=0x39 | 0x2d) => self.get_number(),
            // array
            Some(0x5b) => {
                self.bytes.skip();
                self.get_array()
            }
            // object
            Some(0x7b) => {
                self.bytes.skip();
                self.get_object()
            }
            // false
            Some(0x66) => self.get_false(),
            // true
            Some(0x74) => self.get_true(),
            // null
            Some(0x6e) => self.get_null(),
            _ => Err(Error::parse_err("expect value, found no data")),
        }
    }
}
