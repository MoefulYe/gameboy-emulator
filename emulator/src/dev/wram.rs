use super::bus::{BusDevice, WRAM_LOW_BOUND, WRAM_SIZE};
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
        WorkRam([0; WRAM_SIZE as usize])
    }
}
