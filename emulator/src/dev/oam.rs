use super::bus::{BusDevice, OAM_LOW_BOUND, OAM_SIZE};
use crate::{error::Result, types::Word};
use log::warn;

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
        ObjectAttributeMem([0; OAM_SIZE as usize])
    }
}