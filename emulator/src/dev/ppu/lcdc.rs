use std::ops::{Deref, DerefMut};

use crate::{types::Word, utils::bits::BitMap};

///https://gbdev.io/pandocs/LCDC.html
pub struct LCDControl(Word);

pub const PPU_ENABLE_POS: Word = 7;
pub const WINDOW_MAP_AREA_POS: Word = 6;
pub const WINDOW_ENABLE_POS: Word = 5;
pub const WINDOW_BG_DATA_AREA_POS: Word = 4;
pub const BG_MAP_AREA_POS: Word = 3;
pub const OBJ_SIZE_POS: Word = 2;
pub const OBJ_ENABLE_POS: Word = 1;
pub const WINDOW_BG_ENABLE_POS: Word = 0;

#[repr(u8)]
pub enum MapArea {
    From9800To9BFF = 0,
    From9C00To9FFF = 1,
}

#[repr(u8)]
pub enum DataArea {
    From8800To97FF = 0,
    From8000To8FFF = 1,
}

impl Deref for LCDControl {
    type Target = Word;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LCDControl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl LCDControl {
    pub fn new(data: Word) -> Self {
        Self(data)
    }

    pub fn enabled(&self) -> bool {
        self.test(PPU_ENABLE_POS)
    }

    pub fn window_map_area(&self) -> MapArea {
        unsafe { std::mem::transmute(self.at(WINDOW_MAP_AREA_POS)) }
    }

    pub fn window_enable(&self) -> bool {
        self.test(WINDOW_ENABLE_POS)
    }

    pub fn window_bg_data_area(&self) -> DataArea {
        unsafe { std::mem::transmute(self.at(WINDOW_BG_DATA_AREA_POS)) }
    }

    pub fn bg_map_area(&self) -> MapArea {
        unsafe { std::mem::transmute(self.at(BG_MAP_AREA_POS)) }
    }

    pub fn obj_size(&self) -> (Word, Word) {
        match self.at(OBJ_SIZE_POS) {
            0 => (8, 8),
            _ => (8, 16),
        }
    }

    pub fn window_bg_enable(&self) -> bool {
        self.test(WINDOW_BG_ENABLE_POS)
    }
}
