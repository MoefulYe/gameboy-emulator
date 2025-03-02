use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use super::{
    bus::{HRAM_LOW_BOUND, HRAM_SIZE, WRAM_LOW_BOUND, WRAM_SIZE},
    Reset,
};
use crate::{
    dev::MemoryRegion,
    types::{Addr, Word},
};

#[serde_with::serde_as]
#[derive(Serialize, Deserialize)]
pub struct WRAM(#[serde_as(as = "Box<[_; WRAM_SIZE]>")] Box<[Word; WRAM_SIZE]>);

impl Reset for WRAM {
    fn reset(&mut self) {
        self.0.fill(0)
    }
}

impl MemoryRegion for WRAM {
    fn read(&self, addr: Addr) -> Word {
        *unsafe { self.0.get_unchecked((addr - WRAM_LOW_BOUND) as usize) }
    }

    fn write(&mut self, addr: Addr, data: Word) {
        *unsafe { self.0.get_unchecked_mut((addr - WRAM_LOW_BOUND) as usize) } = data
    }
}

impl Deref for WRAM {
    type Target = [Word; WRAM_SIZE];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for WRAM {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl WRAM {
    pub fn new() -> Self {
        Self(Box::new([0; WRAM_SIZE]))
    }
}

#[serde_with::serde_as]
#[derive(Serialize, Deserialize)]
pub struct HighRam(#[serde_as(as = "Box<[_; HRAM_SIZE]>")] Box<[Word; HRAM_SIZE]>);

impl MemoryRegion for HighRam {
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

impl Deref for HighRam {
    type Target = [Word; HRAM_SIZE];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HighRam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Reset for HighRam {
    fn reset(&mut self) {
        self.0.fill(0)
    }
}
