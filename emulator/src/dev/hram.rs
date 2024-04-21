use super::bus::{BusDevice, HRAM_LOW_BOUND, HRAM_SIZE, OAM_LOW_BOUND};
use crate::types::Word;
use log::warn;

pub struct HighRam([Word; HRAM_SIZE]);

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
        HighRam([0; HRAM_SIZE as usize])
    }
}
