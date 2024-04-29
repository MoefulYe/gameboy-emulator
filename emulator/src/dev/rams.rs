use super::bus::{
    BusDevice, HRAM_LOW_BOUND, HRAM_SIZE, OAM_LOW_BOUND, OAM_SIZE, VRAM_LOW_BOUND, VRAM_SIZE,
    WRAM_LOW_BOUND, WRAM_SIZE,
};
use crate::{error::Result, types::Word};
use log::warn;

pub struct WorkRam([Word; WRAM_SIZE]);

impl BusDevice for WorkRam {
    fn read(&self, addr: crate::types::Addr) -> Result<Word> {
        if let Some(&data) = self.0.get((addr - WRAM_LOW_BOUND) as usize) {
            Ok(data)
        } else {
            warn!("illegal read from wram at address: 0x{addr:04X}");
            Ok(0xFF)
        }
    }

    fn write(&mut self, addr: crate::types::Addr, data: Word) -> Result {
        if let Some(cell) = self.0.get_mut((addr - WRAM_LOW_BOUND) as usize) {
            *cell = data;
            Ok(())
        } else {
            warn!("illegal write to wram at address: 0x{addr:04X}");
            Ok(())
        }
    }
}

impl WorkRam {
    pub fn new() -> Self {
        WorkRam([0; WRAM_SIZE])
    }
}

pub struct VedioRam([Word; VRAM_SIZE]);

impl BusDevice for VedioRam {
    fn read(&self, addr: crate::types::Addr) -> Result<Word> {
        if let Some(&data) = self.0.get((addr - VRAM_LOW_BOUND) as usize) {
            Ok(data)
        } else {
            warn!("illegal read from vram at address: 0x{addr:04X}");
            Ok(0xFF)
        }
    }

    fn write(&mut self, addr: crate::types::Addr, data: Word) -> Result {
        if let Some(cell) = self.0.get_mut((addr - VRAM_LOW_BOUND) as usize) {
            *cell = data;
            Ok(())
        } else {
            warn!("illegal write to vram at address: 0x{addr:04X}");
            Ok(())
        }
    }
}

impl VedioRam {
    pub fn new() -> Self {
        VedioRam([0; VRAM_SIZE])
    }
}

pub struct ObjectAttributeMem([Word; OAM_SIZE]);

impl BusDevice for ObjectAttributeMem {
    fn read(&self, addr: crate::types::Addr) -> Result<Word> {
        if let Some(&data) = self.0.get((addr - OAM_LOW_BOUND) as usize) {
            Ok(data)
        } else {
            warn!("illegal read from oam at address: 0x{addr:04X}");
            Ok(0xFF)
        }
    }

    fn write(&mut self, addr: crate::types::Addr, data: Word) -> Result {
        if let Some(cell) = self.0.get_mut((addr - OAM_LOW_BOUND) as usize) {
            *cell = data;
            Ok(())
        } else {
            warn!("illegal write to oam at address: 0x{addr:04X}");
            Ok(())
        }
    }
}

impl ObjectAttributeMem {
    pub fn new() -> Self {
        ObjectAttributeMem([0; OAM_SIZE])
    }
}

pub struct HighRam([Word; HRAM_SIZE]);

impl BusDevice for HighRam {
    fn read(&self, addr: crate::types::Addr) -> Result<Word> {
        if let Some(&data) = self.0.get((addr - HRAM_LOW_BOUND) as usize) {
            Ok(data)
        } else {
            warn!("illegal read from hram at address: 0x{addr:04X}");
            Ok(0xFF)
        }
    }

    fn write(&mut self, addr: crate::types::Addr, data: Word) -> Result {
        if let Some(cell) = self.0.get_mut((addr - HRAM_LOW_BOUND) as usize) {
            *cell = data;
            Ok(())
        } else {
            warn!("illegal write to hram at address: 0x{addr:04X}");
            Ok(())
        }
    }
}

impl HighRam {
    pub fn new() -> Self {
        HighRam([0; HRAM_SIZE])
    }
}
