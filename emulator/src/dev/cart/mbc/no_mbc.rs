use std::mem::size_of;

use crate::{
    anyerror,
    dev::cart::{
        Rom, RAM_ADDR_HIGH_BOUND, RAM_ADDR_LOW_BOUND, ROM0_ADDR_HIGH_BOUND, ROM0_ADDR_LOW_BOUND,
        ROM1_ADDR_HIGH_BOUND, ROM1_ADDR_LOW_BOUND,
    },
    error::EmuResult,
    types::{Addr, Word},
    utils::bytes::{bytes_to_value, slice_as_bytes},
};
use log::warn;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use super::{RamBank, RomBank, MBC, RAM_BANK_SIZE, ROM_BANK_SIZE};
type RomBanks = [RomBank; 2];
type RamBanks = RamBank;

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct NoMBC {
    #[serde_as(as = "Box<[[_;ROM_BANK_SIZE];2]>")]
    rom: Box<RomBanks>,
    #[serde_as(as = "Option<Box<[_;RAM_BANK_SIZE]>>")]
    ram: Option<Box<RamBanks>>,
}

impl MBC for NoMBC {
    fn read(&self, addr: Addr) -> Word {
        match addr {
            ROM0_ADDR_LOW_BOUND..=ROM0_ADDR_HIGH_BOUND => self.rom[0][addr as usize],
            ROM1_ADDR_LOW_BOUND..=ROM1_ADDR_HIGH_BOUND => self.rom[1][addr as usize - 0x4000],
            RAM_ADDR_LOW_BOUND..=RAM_ADDR_HIGH_BOUND => match &self.ram {
                Some(ram) => ram[(addr - RAM_ADDR_LOW_BOUND) as usize],
                None => {
                    warn!("illegal read no-ram cart at ram area: 0x{addr:04X}");
                    0xFF
                }
            },
            _ => {
                warn!("illegal read cart at address: 0x{addr:04X}");
                0xFF
            }
        }
    }

    fn write(&mut self, addr: Addr, data: Word) {
        match addr {
            ROM0_ADDR_LOW_BOUND..=ROM1_ADDR_HIGH_BOUND => {
                warn!("illegal write rom at address: 0x{addr:04X}")
            }
            RAM_ADDR_LOW_BOUND..=RAM_ADDR_HIGH_BOUND => match &mut self.ram {
                Some(ram) => {
                    ram[(addr - RAM_ADDR_LOW_BOUND) as usize] = data;
                }
                None => {
                    warn!("illegal write no-ram cart at ram area: 0x{addr:04X}");
                }
            },
            _ => warn!("illegal write cart at address: 0x{addr:04X}"),
        }
    }

    fn cart_rom(&self) -> &Rom {
        let rom = self.rom.as_ref();
        slice_as_bytes(rom)
    }

    fn new(rom: Box<[u8]>, ram_size: usize, _: bool, _: i64) -> EmuResult<Self> {
        if rom.len() != size_of::<RomBanks>() {
            return anyerror!(
                "invalid rom size: no mbc cart only allow 32KB rom size, found {rom_size}",
                rom_size = rom.len()
            );
        };
        let rom = unsafe { bytes_to_value::<RomBanks>(rom) };
        let ram = if ram_size == 0 {
            None
        } else if ram_size == size_of::<RamBanks>() {
            Some(Box::new([0; _]))
        } else {
            return anyerror!(
                "invalid ram size: no mbc cart only allow 8KB ram size, found {ram_size}"
            );
        };
        Ok(Self { rom, ram })
    }
}
