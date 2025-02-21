use super::MBC;
use crate::{
    dev::cartridge::{
        RAM_ADDR_HIGH_BOUND, RAM_ADDR_LOW_BOUND, ROM0_ADDR_HIGH_BOUND, ROM0_ADDR_LOW_BOUND,
        ROM1_ADDR_HIGH_BOUND, ROM1_ADDR_LOW_BOUND,
    },
    types::{Addr, Word},
};
pub struct MBC3 {}

impl MBC for MBC3 {
    fn read(&self, addr: Addr) -> Word {
        match addr {
            ROM0_ADDR_LOW_BOUND..=ROM0_ADDR_HIGH_BOUND => std::todo!(),
            ROM1_ADDR_LOW_BOUND..=ROM1_ADDR_HIGH_BOUND => std::todo!(),
            RAM_ADDR_LOW_BOUND..=RAM_ADDR_HIGH_BOUND => std::todo!(),
            _ => std::todo!(),
        }
    }

    fn write(&mut self, addr: Addr, data: Word) {
        match addr {
            ROM0_ADDR_LOW_BOUND..=ROM0_ADDR_HIGH_BOUND => std::todo!(),
            ROM1_ADDR_LOW_BOUND..=ROM1_ADDR_HIGH_BOUND => std::todo!(),
            RAM_ADDR_LOW_BOUND..=RAM_ADDR_HIGH_BOUND => std::todo!(),
            _ => std::todo!(),
        }
    }

    fn ram(&self) -> &[u8] {
        std::todo!()
    }
}
