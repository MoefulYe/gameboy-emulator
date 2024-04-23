use crate::{error::Result, types::Word};

use super::bus::BusDevice;

pub struct IORegs {}

impl BusDevice for IORegs {
    fn read(&self, addr: crate::types::Addr) -> Result<Word> {
        todo!()
    }

    fn write(&mut self, addr: crate::types::Addr, data: Word) -> Result {
        todo!()
    }
}

impl IORegs {
    pub fn new() -> Self {
        Self {}
    }
}
