pub trait Stream<T> {
    fn peek(&mut self) -> Option<&T>;
    fn next(&mut self) -> Option<&T>;
    fn while_if<F>(&mut self, cond: F)
    where
        F: Fn(&T) -> bool;
    fn next_if<F>(&mut self, cond: F) -> bool
    where
        F: Fn(&T) -> bool;
    fn expect(&mut self, it: &[T]) -> bool;
    fn position(&self) -> usize;
    fn finish(&self) -> bool;
    fn skip(&mut self);
    fn skip_by(&mut self, size: usize);
    fn anchor(&mut self);
    fn read(&mut self) -> &[T];
}

pub struct SliceBytes<'a> {
    bytes: &'a [u8],
    pos: usize,
    anchor_point: usize,
    length: usize,
}

impl<'a> SliceBytes<'a> {
    pub fn new(bytes: &'a [u8]) -> SliceBytes {
        SliceBytes {
            bytes,
            pos: 0,
            anchor_point: 0,
            length: bytes.len(),
        }
    }
}

impl<'a> Stream<u8> for SliceBytes<'a> {
    #[inline]
    fn peek(&mut self) -> Option<&u8> {
        if self.finish() {
            None
        } else {
            Some(&self.bytes[self.pos])
        }
    }

    #[inline]
    fn next(&mut self) -> Option<&u8> {
        if self.finish() {
            None
        } else {
            self.pos += 1;
            Some(&self.bytes[self.pos - 1])
        }
    }

    #[inline]
    fn while_if<F>(&mut self, cond: F)
    where
        F: Fn(&u8) -> bool,
    {
        while !self.finish() && cond(&self.bytes[self.pos]) {
            self.pos += 1;
        }
    }

    #[inline]
    fn next_if<F>(&mut self, cond: F) -> bool
    where
        F: Fn(&u8) -> bool,
    {
        if !self.finish() && cond(&self.bytes[self.pos]) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    #[inline]
    fn expect(&mut self, it: &[u8]) -> bool {
        if self.length < self.pos + it.len() {
            return false;
        }
        for i in it {
            if i != &self.bytes[self.pos] {
                return false;
            }
            self.pos += 1;
        }
        true
    }

    #[inline]
    fn position(&self) -> usize {
        self.pos
    }

    #[inline]
    fn finish(&self) -> bool {
        self.length == self.pos
    }

    #[inline]
    fn skip(&mut self) {
        self.pos += 1;
    }

    #[inline]
    fn skip_by(&mut self, size: usize) {
        self.pos += size;
    }

    #[inline]
    fn anchor(&mut self) {
        self.anchor_point = self.pos
    }

    #[inline]
    fn read(&mut self) -> &[u8] {
        &self.bytes[self.anchor_point..self.pos]
    }
}

pub struct StrStream<'a> {
    inner: SliceBytes<'a>,
}

impl<'a> StrStream<'a> {
    pub fn new(str: &'a str) -> StrStream {
        StrStream {
            inner: SliceBytes::new(str.as_bytes()),
        }
    }
}

impl<'a> Stream<u8> for StrStream<'a> {
    #[inline]
    fn peek(&mut self) -> Option<&u8> {
        self.inner.peek()
    }

    #[inline]
    fn next(&mut self) -> Option<&u8> {
        self.inner.next()
    }

    #[inline]
    fn while_if<F>(&mut self, cond: F)
    where
        F: Fn(&u8) -> bool,
    {
        self.inner.while_if(cond)
    }

    #[inline]
    fn next_if<F>(&mut self, cond: F) -> bool
    where
        F: Fn(&u8) -> bool,
    {
        self.inner.next_if(cond)
    }

    #[inline]
    fn expect(&mut self, it: &[u8]) -> bool {
        self.inner.expect(it)
    }

    #[inline]
    fn position(&self) -> usize {
        self.inner.position()
    }

    #[inline]
    fn finish(&self) -> bool {
        self.inner.finish()
    }

    #[inline]
    fn skip(&mut self) {
        self.inner.skip()
    }

    #[inline]
    fn skip_by(&mut self, size: usize) {
        self.inner.skip_by(size)
    }

    #[inline]
    fn anchor(&mut self) {
        self.inner.anchor()
    }

    #[inline]
    fn read(&mut self) -> &[u8] {
        self.inner.read()
    }
}
