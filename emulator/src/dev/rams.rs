use super::{
    bus::{
        HRAM_LOW_BOUND, HRAM_SIZE, OAM_LOW_BOUND, OAM_SIZE, VRAM_LOW_BOUND, VRAM_SIZE,
        WRAM_LOW_BOUND, WRAM_SIZE,
    },
    ppu::{
        graphic::{FlattenRawTiles, RawTiles},
        MapArea, MapAreaType,
    },
};
use crate::{
    dev::BusDevice,
    types::{Addr, Word},
};

pub struct WRAM(pub Box<[Word; WRAM_SIZE]>);

impl Default for WRAM {
    fn default() -> Self {
        Self(Box::new([0; WRAM_SIZE]))
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

pub struct VRAM(pub Box<[Word; VRAM_SIZE]>);

impl Default for VRAM {
    fn default() -> Self {
        Self(Box::new([0; VRAM_SIZE]))
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

    pub fn flatten_tiles_area(&self) -> &FlattenRawTiles {
        unsafe { &*(self as *const Self as *const _) }
    }

    pub fn flatten_tiles_area_mut(&mut self) -> &mut FlattenRawTiles {
        unsafe { &mut *(self as *mut Self as *mut _) }
    }

    pub fn tiles_area(&self) -> &RawTiles {
        unsafe { &*(self as *const Self as *const _) }
    }

    pub fn tiles_area_mut(&mut self) -> &mut RawTiles {
        unsafe { &mut *(self as *mut Self as *mut _) }
    }

    // 0x9800
    pub fn map_area1(&self) -> &MapArea {
        let base = self as *const Self as usize;
        unsafe { &*((base + 0x1800) as *const _) }
    }
    // 0x9C00
    pub fn map_area2(&self) -> &MapArea {
        let base = self as *const Self as usize;
        unsafe { &*((base + 0x1C00) as *const _) }
    }
    pub fn map_area1_mut(&mut self) -> &mut MapArea {
        let base = self as *mut Self as usize;
        unsafe { &mut *((base + 0x1800) as *mut _) }
    }
    pub fn map_area2_mut(&mut self) -> &mut MapArea {
        let base = self as *mut Self as usize;
        unsafe { &mut *((base + 0x1C00) as *mut _) }
    }

    pub fn map_area(&self, area: MapAreaType) -> &MapArea {
        match area {
            MapAreaType::From9800To9BFF => self.map_area1(),
            MapAreaType::From9C00To9FFF => self.map_area2(),
        }
    }

    pub fn map_area_mut(&mut self, area: MapAreaType) -> &mut MapArea {
        match area {
            MapAreaType::From9800To9BFF => self.map_area1_mut(),
            MapAreaType::From9C00To9FFF => self.map_area2_mut(),
        }
    }
}

pub struct OAM(pub Box<[Word; OAM_SIZE]>);

impl Default for OAM {
    fn default() -> Self {
        Self(Box::new([0; OAM_SIZE]))
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

pub struct HighRam(pub Box<[Word; HRAM_SIZE]>);

impl Default for HighRam {
    fn default() -> Self {
        Self(Box::new([0; HRAM_SIZE]))
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
