use super::{
    bus::{HRAM_LOW_BOUND, HRAM_SIZE, WRAM_LOW_BOUND, WRAM_SIZE},
    Reset,
};
use crate::{
    dev::BusDevice,
    types::{Addr, Word},
};

pub struct WRAM(pub Box<[Word; WRAM_SIZE]>);

impl Reset for WRAM {
    fn reset(&mut self) {
        self.0.fill(0)
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
        Self(Box::new([0; WRAM_SIZE]))
    }
}

pub struct HighRam(pub Box<[Word; HRAM_SIZE]>);

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
        Self(Box::new([0; HRAM_SIZE]))
    }
}

impl Reset for HighRam {
    fn reset(&mut self) {
        self.0.fill(0)
    }
}
