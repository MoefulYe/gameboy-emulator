use crate::types::{DWord, Word};

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
