use std::ops::{Deref, DerefMut};

use crate::types::Word;

use super::bit_proxy::BitProxy;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bits(Word);

impl DerefMut for Bits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Bits {
    type Target = Word;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Word> for Bits {
    fn from(value: Word) -> Self {
        Self(value)
    }
}

impl Bits {
    const SET: Word = 1;
    const UNSET: Word = 0;
    #[inline]
    pub fn new(word: Word) -> Self {
        Self(word)
    }

    #[inline]
    pub fn into_word(self) -> Word {
        self.0
    }

    #[inline]
    pub fn at(self, pos: u8) -> Word {
        if self.0 & (1 << pos) != 0 {
            Self::SET
        } else {
            Self::UNSET
        }
    }

    #[inline]
    pub fn proxy_at<'a>(&'a mut self, pos: u8) -> BitProxy<'a> {
        BitProxy::new(&mut self.0, pos)
    }

    #[inline]
    pub fn set(self, pos: u8) -> Self {
        Self(self.0 | (1 << pos))
    }

    #[inline]
    pub fn clear(self, pos: u8) -> Self {
        Self(self.0 & !(1 << pos))
    }

    #[inline]
    pub fn flip(self, pos: u8) -> Self {
        Self(self.0 ^ (1 << pos))
    }

    #[inline]
    pub fn set_value(self, pos: u8, val: bool) -> Self {
        if val {
            self.set(pos)
        } else {
            self.clear(pos)
        }
    }
}
