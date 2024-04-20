use std::ops::{BitAnd, BitOr};

/// 0x0000 - 0x7FFF: 32KB CART ROM
/// 0x8000 - 0x9FFF: 8KB VRAM
/// 0XA000 - 0xBFFF: 8KB CART RAM
/// 0xC000 - 0xDFFF: 8kB WRAM
/// 0xE000 - 0xFDFF: FORBIDEN
/// 0xFE00 - 0xFE9F: Object Attribute Memory (OAM)
/// 0xFEA0 - 0xFEFF: FORBIDEN
/// 0xFF00 - 0xFF7F: I/O Registers
/// 0xFF80 - 0xFFFE: Func Call Stack
/// 0xFFFF - 0xFFFF: Interrupt Enable Register
/// 读取非法地址返回0xFF
/// 写操作非法地址不做任何操作
pub struct Bus {}

#[derive(Debug, Clone, Copy, Eq)]
pub struct Permission(u8);

impl BitOr for Permission {
    type Output = Self;

    fn bitor(self, Self(inner): Self) -> Self::Output {
        Self(self.0 | inner)
    }
}

impl BitAnd for Permission {
    type Output = Self;

    fn bitand(self, Self(inner): Self) -> Self::Output {
        Self(self.0 & inner)
    }
}

impl PartialEq for Permission {
    fn eq(&self, other: &Self) -> bool {
        (*self & Self::MASK).0 == (*other & Self::MASK).0
    }
}

impl Permission {
    pub const READABLE: Self = Self(0b01);
    pub const WRITABLE: Self = Self(0b10);
    pub const READABLE_WRITABLE: Self = Self(0b11);
    pub const FORBIDEN: Self = Self(0b00);
    pub const READONLY: Self = Self::READABLE;
    pub const WRITEONLY: Self = Self::WRITABLE;
    pub const MASK: Self = Self(0b11);

    pub fn new(readable: bool, writable: bool) -> Self {
        let mut permission = 0;
        if readable {
            permission |= Self::READABLE.0;
        }
        if writable {
            permission |= Self::WRITABLE.0;
        }
        Self(permission)
    }

    #[inline]
    pub fn readable(self) -> bool {
        self & Self::READABLE != Self::FORBIDEN
    }

    #[inline]
    pub fn writable(self) -> bool {
        self & Self::WRITABLE != Self::FORBIDEN
    }
}
