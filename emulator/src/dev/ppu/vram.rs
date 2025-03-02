use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::{
    dev::{
        bus::{VRAM_LOW_BOUND, VRAM_SIZE},
        MemoryRegion, Reset,
    },
    types::{Addr, Word},
};

use super::{
    graphic::{RawTileMatrix, RawTiles},
    MapArea, MapAreaType,
};

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct VRAM(#[serde_as(as = "Box<[_; VRAM_SIZE]>")] Box<[Word; VRAM_SIZE]>);

impl Reset for VRAM {
    fn reset(&mut self) {
        self.fill(0)
    }
}

impl Deref for VRAM {
    type Target = Box<[Word; VRAM_SIZE]>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for VRAM {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl MemoryRegion for VRAM {
    fn read(&self, addr: Addr) -> Word {
        *unsafe { self.0.get_unchecked((addr - VRAM_LOW_BOUND) as usize) }
    }

    fn write(&mut self, addr: Addr, data: Word) {
        *unsafe { self.0.get_unchecked_mut((addr - VRAM_LOW_BOUND) as usize) } = data
    }
}

impl VRAM {
    pub fn new() -> Self {
        Self(Box::new([0; VRAM_SIZE]))
    }

    pub fn tiles_area(&self) -> &RawTiles {
        unsafe { &*(self.0.as_ptr() as *const _) }
    }

    pub fn tiles_area_mut(&mut self) -> &mut RawTiles {
        unsafe { &mut *(self.0.as_mut_ptr() as *mut _) }
    }

    pub fn tiles_matrix(&self) -> &RawTileMatrix {
        unsafe { &*(self.0.as_ptr() as *const _) }
    }

    pub fn tiles_matrix_mut(&mut self) -> &mut RawTileMatrix {
        unsafe { &mut *(self.0.as_mut_ptr() as *mut _) }
    }

    // 0x9800
    pub fn map_area1(&self) -> &MapArea {
        let base = self.0.as_ptr() as usize;
        unsafe { &*((base + 0x1800) as *const _) }
    }
    // 0x9C00
    pub fn map_area2(&self) -> &MapArea {
        let base = self.0.as_ptr() as usize;
        unsafe { &*((base + 0x1C00) as *const _) }
    }
    pub fn map_area1_mut(&mut self) -> &mut MapArea {
        let base = self.0.as_mut_ptr() as usize;
        unsafe { &mut *((base + 0x1800) as *mut _) }
    }
    pub fn map_area2_mut(&mut self) -> &mut MapArea {
        let base = self.0.as_mut_ptr() as usize;
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
