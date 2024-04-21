use super::bus::{BusDevice, WRAM_LOW_BOUND, WRAM_SIZE};
use crate::types::Word;
use log::warn;

pub struct WorkRam([Word; WRAM_SIZE]);

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
        WorkRam([0; WRAM_SIZE as usize])
    }
}
