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
        self.bytes.get(self.pos)
    }

    #[inline]
    fn next(&mut self) -> Option<&u8> {
        self.pos += 1;
        self.bytes.get(self.pos - 1)
    }

    #[inline]
    fn while_if<F>(&mut self, cond: F)
    where
        F: Fn(&u8) -> bool,
    {
        while self.next_if(&cond) {}
    }

    #[inline]
    fn next_if<F>(&mut self, cond: F) -> bool
    where
        F: Fn(&u8) -> bool,
    {
        if let Some(b) = self.bytes.get(self.pos) {
            if cond(b) {
                self.pos += 1;
                return true;
            }
        }
        false
    }

    #[inline]
    fn expect(&mut self, it: &[u8]) -> bool {
        if self.length < self.pos + it.len() || it != &self.bytes[self.pos..self.pos + it.len()] {
            false
        } else {
            self.pos += it.len();
            true
        }
    }

    #[inline]
    fn position(&self) -> usize {
        self.pos
    }

    #[inline]
    fn finish(&self) -> bool {
        self.length <= self.pos
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
