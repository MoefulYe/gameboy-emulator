use crate::{
    dev::cartridge::{
        RAM_ADDR_HIGH_BOUND, RAM_ADDR_LOW_BOUND, ROM0_ADDR_HIGH_BOUND, ROM0_ADDR_LOW_BOUND,
        ROM1_ADDR_HIGH_BOUND, ROM1_ADDR_LOW_BOUND,
    },
    error::EmuResult,
    types::{Addr, Word},
};
use log::warn;

use super::{RamBank, RomBank, MBC};

pub struct NoMBC {
    rom: Box<[RomBank; 2]>,
    ram: Box<RamBank>,
}

impl MBC for NoMBC {
    fn read(&self, addr: Addr) -> EmuResult<Word> {
        let data = match addr {
            ROM0_ADDR_LOW_BOUND..=ROM0_ADDR_HIGH_BOUND => self.rom[0][addr as usize],
            ROM1_ADDR_LOW_BOUND..=ROM1_ADDR_HIGH_BOUND => self.rom[1][addr as usize - 0x4000],
            RAM_ADDR_LOW_BOUND..=RAM_ADDR_HIGH_BOUND => self.ram[addr as usize - 0xA000],
            _ => {
                warn!("illegal read cart at address: 0x{addr:04X}");
                0xFF
            }
        };
        Ok(data)
    }

    fn write(&mut self, addr: Addr, data: Word) -> EmuResult {
        match addr {
            ROM0_ADDR_LOW_BOUND..=ROM1_ADDR_HIGH_BOUND => {
                warn!("illegal write rom at address: 0x{addr:04X}")
            }
            RAM_ADDR_LOW_BOUND..=RAM_ADDR_HIGH_BOUND => self.ram[addr as usize - 0xA000] = data,
            _ => warn!("illegal write cart at address: 0x{addr:04X}"),
        }
        Ok(())
    }

    fn ram(&self) -> &[u8] {
        std::todo!()
    }
}
