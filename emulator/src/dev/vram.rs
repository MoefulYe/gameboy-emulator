use super::bus::{BusDevice, VRAM_LOW_BOUND, VRAM_SIZE};
use crate::{error::Result, types::Word};
use log::warn;

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
        VedioRam([0; VRAM_SIZE as usize])
    }
}
