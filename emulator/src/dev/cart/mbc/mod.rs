use log::warn;

use super::{
    Rom, RAM_ADDR_HIGH_BOUND, RAM_ADDR_LOW_BOUND, ROM0_ADDR_HIGH_BOUND, ROM0_ADDR_LOW_BOUND,
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
pub mod rtc;

const KB: usize = 1024;

pub const ROM_BANK_SIZE: usize = 16 * KB;
pub const RAM_BANK_SIZE: usize = 8 * KB;

pub type RomBank = [Word; ROM_BANK_SIZE];
pub type RamBank = [Word; RAM_BANK_SIZE];

pub trait MBC: Sized {
    fn new(rom: Box<[u8]>, ram_size: usize, has_rtc: bool, timestamp: i64) -> EmuResult<Self>;

    fn read(&self, addr: Addr) -> Word {
        match addr {
            ROM0_ADDR_LOW_BOUND..=ROM0_ADDR_HIGH_BOUND => todo!(),
            ROM1_ADDR_LOW_BOUND..=ROM1_ADDR_HIGH_BOUND => todo!(),
            RAM_ADDR_LOW_BOUND..=RAM_ADDR_HIGH_BOUND => todo!(),
            _ => {
                warn!("illegal read cart at address: 0x{addr:04X}");
                0xFF
            }
        }
    }

    fn write(&mut self, addr: Addr, data: Word) {
        match addr {
            ROM0_ADDR_LOW_BOUND..=ROM0_ADDR_HIGH_BOUND => todo!(),
            ROM1_ADDR_LOW_BOUND..=ROM1_ADDR_HIGH_BOUND => todo!(),
            RAM_ADDR_LOW_BOUND..=RAM_ADDR_HIGH_BOUND => todo!(),
            _ => {
                warn!("illegal write cart at address: 0x{addr:04X}");
            }
        }
    }

    fn cart_rom(&self) -> &Rom;
}
