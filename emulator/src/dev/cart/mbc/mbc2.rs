use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use super::{RomBank, MBC, ROM_BANK_SIZE};
use crate::{
    dev::cart::{
        Rom, RAM_ADDR_HIGH_BOUND, RAM_ADDR_LOW_BOUND, ROM0_ADDR_HIGH_BOUND, ROM0_ADDR_LOW_BOUND,
        ROM1_ADDR_HIGH_BOUND, ROM1_ADDR_LOW_BOUND,
    },
    error::EmuResult,
    types::{Addr, Word},
    utils::bytes::{bytes_to_slice, slice_as_bytes},
};

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct MBC2 {
    #[serde_as(as = "Box<[[_; ROM_BANK_SIZE]]>")]
    pub rom_banks: Box<[RomBank]>,
    /// header 中的ram-size字段为0
    #[serde_as(as = "Box<[_; 512]>")]
    pub ram: Box<[Word; 512]>,
    pub ram_enable: bool,
    pub rom_bank_sel: u8,
}

impl MBC2 {
    fn rom0(&self) -> &RomBank {
        &self.rom_banks[0]
    }

    fn rom1(&self) -> &RomBank {
        &self.rom_banks[self.rom_bank_sel as usize]
    }

    fn set_rom_bank_sel(&mut self, data: Word) {
        let rom_bank_num = self.rom_banks.len();
        let data = data & 0x0F;
        let data = if data == 0 { 1 } else { data };
        let data = if rom_bank_num <= 2 {
            data & 0x01
        } else if rom_bank_num <= 4 {
            data & 0x03
        } else if rom_bank_num <= 8 {
            data & 0x07
        } else {
            data
        };
        self.rom_bank_sel = data;
    }

    fn set_ram_enable(&mut self, data: Word) {
        self.ram_enable = data == 0x0A;
    }
}

impl MBC for MBC2 {
    fn read(&self, addr: Addr) -> Word {
        match addr {
            ROM0_ADDR_LOW_BOUND..=ROM0_ADDR_HIGH_BOUND => {
                self.rom0()[(addr - ROM0_ADDR_LOW_BOUND) as usize]
            }
            ROM1_ADDR_LOW_BOUND..=ROM1_ADDR_HIGH_BOUND => {
                self.rom1()[(addr - ROM1_ADDR_LOW_BOUND) as usize]
            }
            RAM_ADDR_LOW_BOUND..=RAM_ADDR_HIGH_BOUND => {
                if self.ram_enable {
                    let idx = (addr - RAM_ADDR_LOW_BOUND) % 512;
                    (self.ram[idx as usize] & 0x0F) | 0xF0
                } else {
                    0xFF
                }
            }
            _ => {
                log::warn!("illegal read cart at address: 0x{addr:04X}");
                0xFF
            }
        }
    }

    fn write(&mut self, addr: Addr, data: Word) {
        match addr {
            0x0000..=0x3FFF => {
                if addr & 0x100 != 0 {
                    self.set_rom_bank_sel(data)
                } else {
                    self.set_ram_enable(data)
                }
            }

            RAM_ADDR_LOW_BOUND..=RAM_ADDR_HIGH_BOUND => {
                if self.ram_enable {
                    let idx = (addr - RAM_ADDR_LOW_BOUND) % 512;
                    self.ram[idx as usize] = data & 0x0F;
                }
            }
            _ => {
                log::warn!("illegal write cart at address: 0x{addr:04X}");
            }
        }
    }

    fn cart_rom(&self) -> &Rom {
        let rom = self.rom_banks.as_ref();
        slice_as_bytes(rom)
    }

    fn new(rom: Box<[u8]>, _: usize, _: bool, _: i64) -> EmuResult<Self> {
        let rom_banks: Box<[RomBank]> = unsafe { bytes_to_slice(rom) };
        let ram = Box::new([0; _]);
        Ok(Self {
            rom_banks,
            ram,
            ram_enable: false,
            rom_bank_sel: 1,
        })
    }
}
