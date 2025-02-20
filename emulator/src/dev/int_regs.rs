use crate::types::{Addr, Word};
use crate::utils::bits::BitMap;

pub const INT_JOYPAD: Word = 4;
pub const INT_SERIAL: Word = 3;
pub const INT_TIMER: Word = 2;
pub const INT_LCD_STAT: Word = 1;
pub const INT_VBLANK: Word = 0;

#[allow(unused)]
pub const INT_JOYPAD_MASK: Word = 1 << INT_JOYPAD;
pub const INT_SERIAL_MASK: Word = 1 << INT_SERIAL;
pub const INT_TIMER_MASK: Word = 1 << INT_TIMER;
pub const INT_LCD_STAT_MASK: Word = 1 << INT_LCD_STAT;
pub const INT_VBLANK_MASK: Word = 1 << INT_VBLANK;

pub type IRQ = Word;
pub const IRQ_JOYPAD: IRQ = 1 << INT_JOYPAD;
pub const IRQ_SERIAL: IRQ = 1 << INT_SERIAL;
pub const IRQ_TIMER: IRQ = 1 << INT_TIMER;
pub const IRQ_LCD_STAT: IRQ = 1 << INT_LCD_STAT;
pub const IRQ_VBLANK: IRQ = 1 << INT_VBLANK;
pub const IRQ_NONE: IRQ = 0;

pub const INT_JOYPAD_ENTRY: Addr = 0x60;
pub const INT_SERIAL_ENTRY: Addr = 0x58;
pub const INT_TIMER_ENTRY: Addr = 0x50;
pub const INT_LCD_STAT_ENTRY: Addr = 0x48;
pub const INT_VBLANK_ENTRY: Addr = 0x40;

pub const INTERRUPT_FLAG_REGISTER_ADDR: Addr = 0xFF0F;
pub const INTERRUPT_MASK_REGISTER_ADDR: Addr = 0xFFFF;

#[derive(Clone, Copy)]
pub struct InterruptMaskRegsiter(Word);

impl Default for InterruptMaskRegsiter {
    fn default() -> Self {
        Self(0x00)
    }
}

impl InterruptMaskRegsiter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn read(self) -> Word {
        self.0
    }

    pub fn write(&mut self, new_val: Word) {
        self.0 = new_val;
    }

    #[inline]
    pub fn val(self) -> Word {
        self.0
    }
}

#[derive(Clone, Copy)]
pub struct InterruptFlagRegister(Word);

impl Default for InterruptFlagRegister {
    fn default() -> Self {
        Self(0x00)
    }
}

impl InterruptFlagRegister {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn read(self) -> Word {
        self.0
    }

    pub fn write(&mut self, new_val: Word) {
        self.0 = new_val;
    }

    pub fn val(self) -> Word {
        self.0
    }

    pub fn add(&mut self, irq: IRQ) {
        self.0 |= irq
    }

    fn clear_at(&mut self, pos: u8) {
        self.0 = self.0.clear_at(pos)
    }

    pub fn clear_timer_int(&mut self) {
        self.clear_at(INT_TIMER)
    }

    pub fn clear_serial_int(&mut self) {
        self.clear_at(INT_SERIAL)
    }

    pub fn clear_joypad_int(&mut self) {
        self.clear_at(INT_JOYPAD)
    }

    pub fn clear_lcd_stat_int(&mut self) {
        self.clear_at(INT_LCD_STAT)
    }

    pub fn clear_vblank_int(&mut self) {
        self.clear_at(INT_VBLANK)
    }
}
