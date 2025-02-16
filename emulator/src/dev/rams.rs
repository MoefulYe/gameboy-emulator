use super::bus::{
    HRAM_LOW_BOUND, HRAM_SIZE, OAM_LOW_BOUND, OAM_SIZE, VRAM_LOW_BOUND, VRAM_SIZE, WRAM_LOW_BOUND,
    WRAM_SIZE,
};
use crate::{
    dev::BusDevice,
    types::{Addr, Word},
};
pub struct WRAM([Word; WRAM_SIZE]);

impl Default for WRAM {
    fn default() -> Self {
        Self([0; WRAM_SIZE])
    }
}

impl BusDevice for WRAM {
    fn read(&self, addr: Addr) -> Word {
        *unsafe { self.0.get_unchecked((addr - WRAM_LOW_BOUND) as usize) }
    }

    fn write(&mut self, addr: Addr, data: Word) {
        *unsafe { self.0.get_unchecked_mut((addr - WRAM_LOW_BOUND) as usize) } = data
    }
}

impl WRAM {
    pub fn new() -> Self {
        Default::default()
    }
}

pub struct VRAM([Word; VRAM_SIZE]);

impl Default for VRAM {
    fn default() -> Self {
        Self([0; VRAM_SIZE])
    }
}

impl BusDevice for VRAM {
    fn read(&self, addr: Addr) -> Word {
        *unsafe { self.0.get_unchecked((addr - VRAM_LOW_BOUND) as usize) }
    }

    fn write(&mut self, addr: Addr, data: Word) {
        *unsafe { self.0.get_unchecked_mut((addr - VRAM_LOW_BOUND) as usize) } = data
    }
}

impl VRAM {
    pub fn new() -> Self {
        Default::default()
    }
}

pub struct OAM([Word; OAM_SIZE]);

impl Default for OAM {
    fn default() -> Self {
        Self([0; OAM_SIZE])
    }
}

impl BusDevice for OAM {
    fn read(&self, addr: Addr) -> Word {
        *unsafe { self.0.get_unchecked((addr - OAM_LOW_BOUND) as usize) }
    }

    fn write(&mut self, addr: Addr, data: Word) {
        *unsafe { self.0.get_unchecked_mut((addr - OAM_LOW_BOUND) as usize) } = data
    }
}

impl OAM {
    pub fn new() -> Self {
        Default::default()
    }
}

pub struct HighRam([Word; HRAM_SIZE]);

impl Default for HighRam {
    fn default() -> Self {
        Self([0; HRAM_SIZE])
    }
}

impl BusDevice for HighRam {
    fn read(&self, addr: Addr) -> Word {
        *unsafe { self.0.get_unchecked((addr - HRAM_LOW_BOUND) as usize) }
    }

    fn write(&mut self, addr: Addr, data: Word) {
        *unsafe { self.0.get_unchecked_mut((addr - HRAM_LOW_BOUND) as usize) } = data
    }
}

impl HighRam {
    pub fn new() -> Self {
        Default::default()
    }
}
