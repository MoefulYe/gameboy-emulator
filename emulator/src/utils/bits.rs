use crate::types::{DWord, Word};

pub struct BitProxy<'a> {
    byte: &'a mut Word,
    pos: u8,
}

impl<'a> BitProxy<'a> {
    pub fn new(byte: &'a mut Word, pos: u8) -> Self {
        BitProxy { byte, pos }
    }

    pub fn get(&self) -> bool {
        (*self.byte & (1 << self.pos)) != 0
    }

    pub fn set(&mut self) {
        *self.byte |= 1 << self.pos;
    }

    pub fn flip(&mut self) {
        *self.byte ^= 1 << self.pos;
    }

    pub fn clear(&mut self) {
        *self.byte &= !(1 << self.pos);
    }

    pub fn set_value(&mut self, val: bool) {
        if val {
            self.set();
        } else {
            self.clear();
        }
    }
}

pub trait BitMap {
    type AtType;
    fn empty() -> Self;
    fn at(self, pos: Self) -> Self::AtType;
    fn set_at(self, pos: Self) -> Self;
    fn clear_at(self, pos: Self) -> Self;
    fn set_at_with(self, pos: Self, val: bool) -> Self;
}

impl BitMap for Word {
    type AtType = Self;
    #[inline]
    fn empty() -> Self {
        0
    }

    #[inline]
    fn at(self, pos: Self) -> Self::AtType {
        if self & (1 << pos) != 0 {
            1
        } else {
            0
        }
    }

    #[inline]
    fn set_at(self, pos: Self) -> Self {
        self | (1 << pos)
    }

    #[inline]
    fn clear_at(self, pos: Self) -> Self {
        self & !(1 << pos)
    }

    #[inline]
    fn set_at_with(self, pos: Self, val: bool) -> Self {
        if val {
            self.set_at(pos)
        } else {
            self.clear_at(pos)
        }
    }
}

impl BitMap for DWord {
    type AtType = bool;
    #[inline]
    fn empty() -> Self {
        0
    }

    #[inline]
    fn at(self, pos: Self) -> Self::AtType {
        self & (1 << pos) != 0
    }

    #[inline]
    fn set_at(self, pos: Self) -> Self {
        self | (1 << pos)
    }

    #[inline]
    fn clear_at(self, pos: Self) -> Self {
        self & !(1 << pos)
    }

    #[inline]
    fn set_at_with(self, pos: Self, val: bool) -> Self {
        if val {
            self.set_at(pos)
        } else {
            self.clear_at(pos)
        }
    }
}
