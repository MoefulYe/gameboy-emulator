use log::warn;

use super::{
    RAM_ADDR_HIGH_BOUND, RAM_ADDR_LOW_BOUND, ROM0_ADDR_HIGH_BOUND, ROM0_ADDR_LOW_BOUND,
    ROM1_ADDR_HIGH_BOUND, ROM1_ADDR_LOW_BOUND,
};
use crate::{
    error::EmuResult,
    types::{Addr, Word},
};

pub mod mbc1;
pub mod mbc2;
pub mod mbc3;
pub mod no_mbc;

const KB: usize = 1024;

const ROM_BANK_SIZE: usize = 16 * KB;
const RAM_BANK_SIZE: usize = 8 * KB;

pub type RomBank = [Word; ROM_BANK_SIZE];
pub type RamBank = [Word; RAM_BANK_SIZE];

pub trait MBC {
    fn read(&self, addr: Addr) -> EmuResult<Word> {
        match addr {
            ROM0_ADDR_LOW_BOUND..=ROM0_ADDR_HIGH_BOUND => todo!(),
            ROM1_ADDR_LOW_BOUND..=ROM1_ADDR_HIGH_BOUND => todo!(),
            RAM_ADDR_LOW_BOUND..=RAM_ADDR_HIGH_BOUND => todo!(),
            _ => {
                warn!("illegal read cart at address: 0x{addr:04X}");
                Ok(0xFF)
            }
        }
    }
    fn write(&mut self, addr: Addr, data: Word) -> EmuResult {
        match addr {
            ROM0_ADDR_LOW_BOUND..=ROM0_ADDR_HIGH_BOUND => todo!(),
            ROM1_ADDR_LOW_BOUND..=ROM1_ADDR_HIGH_BOUND => todo!(),
            RAM_ADDR_LOW_BOUND..=RAM_ADDR_HIGH_BOUND => todo!(),
            _ => {
                warn!("illegal write cart at address: 0x{addr:04X}");
                Ok(())
            }
        }
    }
    fn ram(&self) -> &[u8] {
        todo!()
    }
}
