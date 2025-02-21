use super::MBC;
use crate::{
    dev::cartridge::{
        RAM_ADDR_HIGH_BOUND, RAM_ADDR_LOW_BOUND, ROM0_ADDR_HIGH_BOUND, ROM0_ADDR_LOW_BOUND,
        ROM1_ADDR_HIGH_BOUND, ROM1_ADDR_LOW_BOUND,
    },
    types::{Addr, Word},
};

pub struct MBC2 {}

impl MBC for MBC2 {
    fn read(&self, addr: Addr) -> Word {
        todo!()
    }

    fn write(&mut self, addr: Addr, data: Word) {
        todo!()
    }

    fn ram(&self) -> &[u8] {
        todo!()
    }
}
