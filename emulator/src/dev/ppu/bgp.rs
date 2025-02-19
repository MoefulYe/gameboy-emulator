use std::ops::{Deref, DerefMut};

use crate::types::Word;

#[derive(Clone, Copy)]
pub struct Palette(pub Word);

impl Deref for Palette {
    type Target = Word;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Palette {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Palette {
    pub fn apply(&self, color: Word) -> Word {
        match color {
            0b00 => self.0 & 0b11,
            0b01 => (self.0 >> 2) & 0b11,
            0b10 => (self.0 >> 4) & 0b11,
            0b11 => (self.0 >> 6) & 0b11,
            _ => unreachable!(),
        }
    }

    pub fn read(&self) -> Word {
        self.0
    }

    pub fn write(&mut self, data: Word) {
        self.0 = data
    }
}
