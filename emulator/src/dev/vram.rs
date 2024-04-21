use super::bus::{BusDevice, VRAM_LOW_BOUND, VRAM_SIZE};
use crate::types::Word;
use log::warn;

pub struct VedioRam([Word; VRAM_SIZE]);

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
        VedioRam([0; VRAM_SIZE as usize])
    }
}
