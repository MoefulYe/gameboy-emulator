use log::warn;

use super::{RamBank, RomBank, MBC, ROM_BANK_SIZE};
use crate::dev::cartridge::{
    RAM_ADDR_HIGH_BOUND, RAM_ADDR_LOW_BOUND, ROM0_ADDR_HIGH_BOUND, ROM0_ADDR_LOW_BOUND,
    ROM1_ADDR_HIGH_BOUND, ROM1_ADDR_LOW_BOUND,
};
use crate::error::EmulatorError::RomBankNoExisted;
use crate::error::{EmuErr, EmuResult};
use crate::types::{Addr, Word};

#[derive(PartialEq)]
enum WorkingMode {
    Default,
    Advanced,
}

pub struct MBC1 {
    rom_banks: Box<[RomBank]>,
    ram_banks: Box<[RamBank]>,
    rom1_bank_sel: Word,
    ram_bank_sel: Word,
    ram_enable: bool,
    mode: WorkingMode,
}

impl MBC for MBC1 {
    fn write(&mut self, addr: Addr, data: Word) -> EmuResult {
        todo!()
    }

    fn ram(&self) -> &[u8] {
        todo!()
    }

    fn read(&self, addr: Addr) -> EmuResult<Word> {
        match addr {
            ROM0_ADDR_LOW_BOUND..=ROM0_ADDR_HIGH_BOUND => {
                if self.mode == WorkingMode::Advanced && self.ram_bank_sel > 32 {
                    let bank_idx = self.ram_bank_sel << 5;
                    match self.rom_banks.get(bank_idx as usize) {
                        Some(bank) => Ok(bank[addr as usize]),
                        None => EmuErr(RomBankNoExisted { bank_idx }),
                    }
                } else {
                    Ok(self.rom_banks[0][addr as usize])
                }
            }
            ROM1_ADDR_LOW_BOUND..=ROM1_ADDR_HIGH_BOUND => {
                if self.mode == WorkingMode::Advanced && self.ram_bank_sel > 32 {
                    let bank_idx = self.ram_bank_sel << 5 + self.rom1_bank_sel;
                    match self.rom_banks.get(bank_idx as usize) {
                        Some(bank) => Ok(bank[(addr - ROM1_ADDR_LOW_BOUND) as usize]),
                        None => EmuErr(RomBankNoExisted { bank_idx }),
                    }
                } else {
                    match self.rom_banks.get(self.rom1_bank_sel as usize) {
                        Some(bank) => Ok(bank[(addr - ROM1_ADDR_LOW_BOUND) as usize]),
                        None => EmuErr(RomBankNoExisted {
                            bank_idx: self.rom1_bank_sel,
                        }),
                    }
                }
            }
            RAM_ADDR_LOW_BOUND..=RAM_ADDR_HIGH_BOUND => {
                if self.ram_enable {
                    if self.rom1_bank_sel <= 32 && self.mode == WorkingMode::Advanced {
                        match self.ram_banks[] {

                        }
                    } else {

                    }
                } else {
                    Ok(0xFF)
                }
            }
            _ => std::todo!(),
        }
    }
}
