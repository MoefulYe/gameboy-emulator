use crate::error::Result;
use crate::types::Word;

pub const JOYPAD: Word = 4;
pub const SERIAL: Word = 3;
pub const TIMER: Word = 2;
pub const LCD_STAT: Word = 1;
pub const V_BLANK: Word = 0;

#[derive(Clone, Copy)]
pub struct InterruptEnableRegsiter(Word);

impl InterruptEnableRegsiter {
    pub fn new() -> Self {
        InterruptEnableRegsiter(0x00)
    }

    pub fn read(self) -> Result<Word> {
        Ok(self.0)
    }

    pub fn write(&mut self, new_val: Word) -> Result<()> {
        self.0 = new_val;
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct InterruptMaskRegister(Word);

impl InterruptMaskRegister {
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
}
