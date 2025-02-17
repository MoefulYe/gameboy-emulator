use std::ops::{Deref, DerefMut};

use crate::types::Word;

pub struct BGP(pub Word);

impl Deref for BGP {
    type Target = Word;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BGP {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl BGP {
    pub fn apply(&self, color: Word) -> Word {
        match color {
            0b00 => self.0 & 0b11,
            0b01 => (self.0 >> 2) & 0b11,
            0b10 => (self.0 >> 4) & 0b11,
            0b11 => (self.0 >> 6) & 0b11,
            _ => unreachable!(),
        }
    }
}
