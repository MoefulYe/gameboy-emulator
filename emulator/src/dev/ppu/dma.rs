use serde::{Deserialize, Serialize};

use crate::types::Word;

#[derive(Serialize, Deserialize)]
pub struct DMA {
    active: bool,
    base: Word,
    offset: Word,
    start_delay: Word,
    ticks: u8,
}

impl Default for DMA {
    fn default() -> Self {
        Self {
            active: false,
            base: 0,
            offset: 0,
            start_delay: 0,
            ticks: u8::MAX,
        }
    }
}

const OFFSET_END: Word = 0xA0;

impl DMA {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn read(&self) -> Word {
        self.base
    }

    pub fn write(&mut self, data: Word) {
        self.base = data;
        self.active = true;
        self.offset = 0;
        self.start_delay = 1;
    }

    pub fn tick(&mut self) -> Option<(Word, Word)> {
        self.ticks = self.ticks.wrapping_add(1);
        if self.ticks % 4 != 0 || !self.active {
            return None;
        }
        if self.start_delay > 0 {
            self.start_delay -= 1;
            return None;
        }
        let ret = (self.base, self.offset);
        self.offset += 1;
        self.active = self.offset < OFFSET_END;
        Some(ret)
    }
}
