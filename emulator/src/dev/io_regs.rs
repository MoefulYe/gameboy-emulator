use super::bus::BusDevice;

pub struct IORegs {}

impl BusDevice for IORegs {
    fn read(&self, addr: crate::types::Addr) -> crate::types::Word {
        todo!()
    }

    fn write(&mut self, addr: crate::types::Addr, data: crate::types::Word) {
        todo!()
    }
}

impl IORegs {
    pub fn new() -> Self {
        Self {}
    }
}
