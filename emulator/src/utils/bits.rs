use super::bit_proxy::BitProxy;
use crate::types::Word;

pub trait BitMap {
    fn empty() -> Self;
    fn at(self, pos: u8) -> Word;
    fn set_at(self, pos: u8) -> Self;
    fn clear_at(self, pos: u8) -> Self;
    fn at_mut<'a>(&'a mut self, pos: u8) -> BitProxy<'a>;
    fn set_at_with(self, pos: u8, val: bool) -> Self;
}

impl BitMap for Word {
    #[inline]
    fn empty() -> Self {
        0
    }

    #[inline]
    fn at(self, pos: u8) -> Word {
        if self & (1 << pos) != 0 {
            1
        } else {
            0
        }
    }

    #[inline]
    fn set_at(self, pos: u8) -> Self {
        self | (1 << pos)
    }

    #[inline]
    fn clear_at(self, pos: u8) -> Self {
        self & !(1 << pos)
    }

    #[inline]
    fn at_mut<'a>(&'a mut self, pos: u8) -> BitProxy<'a> {
        BitProxy::new(self, pos)
    }

    #[inline]
    fn set_at_with(self, pos: u8, val: bool) -> Self {
        if val {
            self.set_at(pos)
        } else {
            self.clear_at(pos)
        }
    }
}
