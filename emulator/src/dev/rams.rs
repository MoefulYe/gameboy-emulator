use super::bus::{
    HRAM_LOW_BOUND, HRAM_SIZE, OAM_LOW_BOUND, OAM_SIZE, VRAM_LOW_BOUND, VRAM_SIZE, WRAM_LOW_BOUND,
    WRAM_SIZE,
};
use crate::{dev::BusDevice, types::Word};
use log::warn;

pub struct WorkRam([Word; WRAM_SIZE]);

impl Default for WorkRam {
    fn default() -> Self {
        Self([0; WRAM_SIZE])
    }
}

impl BusDevice for WorkRam {
    fn read(&self, addr: crate::types::Addr) -> Word {
        if let Some(&data) = self.0.get((addr - WRAM_LOW_BOUND) as usize) {
            data
        } else {
            warn!("illegal read from wram at address: 0x{addr:04X}");
            0xFF
        }
    }

    fn write(&mut self, addr: crate::types::Addr, data: Word) {
        if let Some(cell) = self.0.get_mut((addr - WRAM_LOW_BOUND) as usize) {
            *cell = data;
        } else {
            warn!("illegal write to wram at address: 0x{addr:04X}");
        }
    }
}

impl WorkRam {
    pub fn new() -> Self {
        Default::default()
    }
}

pub struct VedioRam([Word; VRAM_SIZE]);

impl Default for VedioRam {
    fn default() -> Self {
        Self([0; VRAM_SIZE])
    }
}

impl BusDevice for VedioRam {
    fn read(&self, addr: crate::types::Addr) -> Word {
        if let Some(&data) = self.0.get((addr - VRAM_LOW_BOUND) as usize) {
            data
        } else {
            warn!("illegal read from vram at address: 0x{addr:04X}");
            0xFF
        }
    }

    fn write(&mut self, addr: crate::types::Addr, data: Word) {
        if let Some(cell) = self.0.get_mut((addr - VRAM_LOW_BOUND) as usize) {
            *cell = data;
        } else {
            warn!("illegal write to vram at address: 0x{addr:04X}");
        }
    }
}

impl VedioRam {
    pub fn new() -> Self {
        Default::default()
    }
}

pub struct ObjectAttributeMem([Word; OAM_SIZE]);

impl Default for ObjectAttributeMem {
    fn default() -> Self {
        Self([0; OAM_SIZE])
    }
}

impl BusDevice for ObjectAttributeMem {
    fn read(&self, addr: crate::types::Addr) -> Word {
        if let Some(&data) = self.0.get((addr - OAM_LOW_BOUND) as usize) {
            data
        } else {
            warn!("illegal read from oam at address: 0x{addr:04X}");
            0xFF
        }
    }

    fn write(&mut self, addr: crate::types::Addr, data: Word) {
        if let Some(cell) = self.0.get_mut((addr - OAM_LOW_BOUND) as usize) {
            *cell = data;
        } else {
            warn!("illegal write to oam at address: 0x{addr:04X}");
        }
    }
}

impl ObjectAttributeMem {
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
    fn read(&self, addr: crate::types::Addr) -> Word {
        if let Some(&data) = self.0.get((addr - HRAM_LOW_BOUND) as usize) {
            data
        } else {
            warn!("illegal read from hram at address: 0x{addr:04X}");
            0xFF
        }
    }

    fn write(&mut self, addr: crate::types::Addr, data: Word) {
        if let Some(cell) = self.0.get_mut((addr - HRAM_LOW_BOUND) as usize) {
            *cell = data;
        } else {
            warn!("illegal write to hram at address: 0x{addr:04X}");
        }
    }
}

impl HighRam {
    pub fn new() -> Self {
        Default::default()
    }
}
