use crate::types::Word;

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
}
