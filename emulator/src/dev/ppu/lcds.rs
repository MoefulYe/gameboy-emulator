use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

use crate::{
    dev::Reset,
    types::Word,
    utils::bits::{BitMap, BitProxy},
};

#[repr(u8)]
#[derive(Debug)]
pub enum WorkMode {
    HBlank = 0,
    VBlank = 1,
    OAMScan = 2,
    Drawing = 3,
}

/// https://gbdev.io/pandocs/STAT.html
#[derive(Serialize, Deserialize)]
pub struct LCDStat(pub Word);

impl Default for LCDStat {
    fn default() -> Self {
        Self(0x2)
    }
}

impl Deref for LCDStat {
    type Target = Word;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LCDStat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl LCDStat {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn write(&mut self, data: Word) {
        self.0 = (data & 0b1111_1000) | (self.0 & 0b0000_0111)
    }

    pub fn mode(&self) -> WorkMode {
        unsafe { std::mem::transmute(self.0 & 0b11) }
    }

    pub fn set_mode(&mut self, mode: WorkMode) {
        self.0 = (self.0 & 0b1111_1100) | (mode as Word)
    }

    pub fn lyc_flag(&mut self) -> BitProxy {
        BitProxy::new(self, 2)
    }

    pub fn hblank_int(&self) -> bool {
        self.test(3)
    }

    pub fn vblank_int(&self) -> bool {
        self.test(4)
    }

    pub fn oam_int(&self) -> bool {
        self.test(5)
    }

    pub fn lyc_int(&self) -> bool {
        self.test(6)
    }
}
