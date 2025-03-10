use super::{RamBank, RomBank, MBC, RAM_BANK_SIZE, ROM_BANK_SIZE};
use crate::dev::cart::{
    Rom, RAM_ADDR_HIGH_BOUND, RAM_ADDR_LOW_BOUND, ROM0_ADDR_HIGH_BOUND, ROM0_ADDR_LOW_BOUND,
    ROM1_ADDR_HIGH_BOUND, ROM1_ADDR_LOW_BOUND,
};
use crate::error::EmuResult;
use crate::types::{Addr, Word};
use crate::utils::bits::BitMap;
use crate::utils::bytes::{bytes_to_slice, slice_as_bytes};
use log::{debug, error, warn};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub enum WorkingMode {
    Simple,
    Advanced,
}

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct MBC1 {
    #[serde_as(as = "Box<[[_; ROM_BANK_SIZE]]>")]
    pub rom_banks: Box<[RomBank]>,
    #[serde_as(as = "Box<[[_; RAM_BANK_SIZE]]>")]
    pub ram_banks: Box<[RamBank]>,
    pub rom_bank_sel: u8,
    pub ram_bank_sel: u8,
    pub ram_enable: bool,
    pub mode: WorkingMode,
}

impl MBC1 {
    fn no_ram(&self) -> bool {
        self.rom_banks.is_empty()
    }

    fn has_ram(&self) -> bool {
        !self.rom_banks.is_empty()
    }

    fn rom0(&self) -> &RomBank {
        if self.mode == WorkingMode::Advanced && self.rom_banks.len() > 32 {
            let bank_idx = self.ram_bank_sel << 5;
            &self.rom_banks[bank_idx as usize]
        } else {
            &self.rom_banks[0]
        }
    }

    fn rom1(&self) -> &RomBank {
        if self.mode == WorkingMode::Advanced && self.rom_banks.len() > 32 {
            let bank_idx = self.ram_bank_sel << 5 | self.rom_bank_sel;
            &self.rom_banks[bank_idx as usize]
        } else {
            &self.rom_banks[self.rom_bank_sel as usize]
        }
    }

    fn ram(&self) -> Option<&RamBank> {
        if self.no_ram() {
            return None;
        }
        let bank = if self.rom_banks.len() <= 32 && self.mode == WorkingMode::Advanced {
            &self.ram_banks[self.ram_bank_sel as usize]
        } else {
            &self.ram_banks[0]
        };
        // match self.mode {
        //     WorkingMode::Simple => &self.ram_banks[self.ram_bank_sel as usize],
        //     WorkingMode::Advanced => &self.ram_banks[0],
        // };
        Some(bank)
    }

    fn ram_mut(&mut self) -> Option<&mut RamBank> {
        if self.no_ram() {
            return None;
        }
        // let bank = match self.mode {
        //     WorkingMode::Simple => &mut self.ram_banks[self.ram_bank_sel as usize],
        //     WorkingMode::Advanced => &mut self.ram_banks[0],
        // };
        let bank = if self.rom_banks.len() <= 32 && self.mode == WorkingMode::Advanced {
            &mut self.ram_banks[self.ram_bank_sel as usize]
        } else {
            &mut self.ram_banks[0]
        };
        Some(bank)
    }

    fn set_ram_enable(&mut self, data: Word) {
        if self.has_ram() {
            self.ram_enable = (data & 0x0F) == 0x0A
        }
    }

    fn set_rom_bank_sel(&mut self, data: Word) {
        let rom_banks_num = self.rom_banks.len();
        let data = data & 0x1F;
        let data = if data == 0 { 1 } else { data };
        let data = if rom_banks_num <= 2 {
            data & 0x01
        } else if rom_banks_num <= 4 {
            data & 0x03
        } else if rom_banks_num <= 8 {
            data & 0x07
        } else if rom_banks_num <= 16 {
            data & 0x0F
        } else {
            data
        };
        self.rom_bank_sel = data;
    }

    fn set_ram_bank_sel(&mut self, data: Word) {
        let data = data & 0x03;
        // This second 2-bit register can be used to select a RAM Bank in range from $00–$03 (32 KiB ram carts only)
        // or to specify the upper two bits (bits 5-6) of the ROM Bank number (1 MiB ROM or larger carts only).
        // If neither ROM nor RAM is large enough, setting this register does nothing.
        let data = if self.rom_banks.len() > 32 {
            if self.rom_banks.len() <= 64 {
                data & 0x01
            } else {
                data
            }
        } else {
            if self.ram_banks.len() <= 1 {
                0
            } else if self.ram_banks.len() <= 2 {
                data & 0x01
            } else {
                data
            }
        };
        self.ram_bank_sel = data;
    }

    fn set_mode(&mut self, data: Word) {
        let data = data.test(0);
        // rom大于512KB或者ram大于8KB
        if self.rom_banks.len() > 32 || self.ram_banks.len() > 1 {
            self.mode = match data {
                true => WorkingMode::Advanced,
                false => WorkingMode::Simple,
            }
        }
    }
}

impl MBC for MBC1 {
    fn write(&mut self, addr: Addr, data: Word) {
        match addr {
            0x0000..=0x1FFF => self.set_ram_enable(data),
            0x2000..=0x3FFF => self.set_rom_bank_sel(data),
            0x4000..=0x5FFF => self.set_ram_bank_sel(data),
            0x6000..=0x7FFF => self.set_mode(data),
            RAM_ADDR_LOW_BOUND..=RAM_ADDR_HIGH_BOUND => match (self.ram_enable, self.ram_mut()) {
                (true, Some(ram)) => ram[(addr - RAM_ADDR_LOW_BOUND) as usize] = data,
                (_, None) => warn!("illegal write no-ram cart at ram area: 0x{addr:04X}"),
                _ => {}
            },
            _ => warn!("illegal write cart at address: 0x{addr:04X}"),
        }
    }

    fn read(&self, addr: Addr) -> Word {
        match addr {
            ROM0_ADDR_LOW_BOUND..=ROM0_ADDR_HIGH_BOUND => {
                self.rom0()[(addr - ROM0_ADDR_LOW_BOUND) as usize]
            }
            ROM1_ADDR_LOW_BOUND..=ROM1_ADDR_HIGH_BOUND => {
                self.rom1()[(addr - ROM1_ADDR_LOW_BOUND) as usize]
            }
            RAM_ADDR_LOW_BOUND..=RAM_ADDR_HIGH_BOUND => match (self.ram_enable, self.ram()) {
                (true, Some(ram)) => ram[(addr - RAM_ADDR_LOW_BOUND) as usize],
                (_, None) => {
                    warn!("illegal read no-ram cart at ram area: 0x{addr:04X}");
                    0xFF
                }
                _ => 0xFF,
            },
            _ => {
                warn!("illegal read cart at address: 0x{addr:04X}");
                0xFF
            }
        }
    }

    fn cart_rom(&self) -> &Rom {
        let rom = self.rom_banks.as_ref();
        slice_as_bytes(rom)
    }

    fn new(rom: Box<[u8]>, ram_size: usize, _: bool, _: i64) -> EmuResult<Self> {
        let rom_banks: Box<[RomBank]> = unsafe { bytes_to_slice(rom) };
        let ram_banks_num = ram_size / RAM_BANK_SIZE;
        let ram_banks = vec![[0; RAM_BANK_SIZE]; ram_banks_num].into_boxed_slice();
        Ok(Self {
            rom_banks,
            ram_banks,
            rom_bank_sel: 1,
            ram_bank_sel: 0,
            ram_enable: false,
            mode: WorkingMode::Simple,
        })
    }
}
