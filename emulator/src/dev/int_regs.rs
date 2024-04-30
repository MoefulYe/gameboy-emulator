use crate::error::Result;
use crate::types::{Addr, Word};
use crate::utils::bits::BitMap;

pub const INT_JOYPAD: Word = 4;
pub const INT_SERIAL: Word = 3;
pub const INT_TIMER: Word = 2;
pub const INT_LCD_STAT: Word = 1;
pub const INT_VBLANK: Word = 0;

pub const INT_JOYPAD_MASK: Word = 1 << INT_JOYPAD;
pub const INT_SERIAL_MASK: Word = 1 << INT_SERIAL;
pub const INT_TIMER_MASK: Word = 1 << INT_TIMER;
pub const INT_LCD_STAT_MASK: Word = 1 << INT_LCD_STAT;
pub const INT_VBLANK_MASK: Word = 1 << INT_VBLANK;

pub const INT_JOYPAD_ENTRY: Addr = 0x60;
pub const INT_SERIAL_ENTRY: Addr = 0x58;
pub const INT_TIMER_ENTRY: Addr = 0x50;
pub const INT_LCD_STAT_ENTRY: Addr = 0x48;
pub const INT_VBLANK_ENTRY: Addr = 0x40;

#[derive(Clone, Copy)]
pub struct InterruptMaskRegsiter(Word);

impl InterruptMaskRegsiter {
    pub fn new() -> Self {
        InterruptMaskRegsiter(0x00)
    }

    pub fn read(self) -> Result<Word> {
        Ok(self.0)
    }

    pub fn write(&mut self, new_val: Word) -> Result<()> {
        self.0 = new_val;
        Ok(())
    }

    #[inline]
    pub fn val(self) -> Word {
        self.0
    }

    pub fn set_at(&mut self, pos: u8) {
        self.0 = self.0.set_at(pos)
    }

    pub fn clear_at(&mut self, pos: u8) {
        self.0 = self.0.clear_at(pos)
    }

    pub fn set_at_with(&mut self, pos: u8, val: bool) {
        self.0 = self.0.set_at_with(pos, val)
    }
}

#[derive(Clone, Copy)]
pub struct InterruptFlagRegister(Word);

impl InterruptFlagRegister {
    pub fn new() -> Self {
        Self(0x00)
    }

    pub fn read(self) -> Result<Word> {
        Ok(self.0)
    }

    pub fn write(&mut self, new_val: Word) -> Result<()> {
        self.0 = new_val;
        Ok(())
    }

    #[inline]
    pub fn val(self) -> Word {
        self.0
    }

    pub fn set_at(&mut self, pos: u8) {
        self.0 = self.0.set_at(pos)
    }

    pub fn clear_at(&mut self, pos: u8) {
        self.0 = self.0.clear_at(pos)
    }

    pub fn set_at_with(&mut self, pos: u8, val: bool) {
        self.0 = self.0.set_at_with(pos, val)
    }
}
