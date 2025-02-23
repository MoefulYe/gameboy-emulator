use super::{rtc::RTC, RamBank, RomBank, MBC, RAM_BANK_SIZE, ROM_BANK_SIZE};
use crate::{
    dev::cart::{
        Rom, RAM_ADDR_HIGH_BOUND, RAM_ADDR_LOW_BOUND, ROM0_ADDR_HIGH_BOUND, ROM0_ADDR_LOW_BOUND,
        ROM1_ADDR_HIGH_BOUND, ROM1_ADDR_LOW_BOUND,
    },
    error::EmuResult,
    types::{Addr, Word},
    utils::bytes::{bytes_to_slice, slice_as_bytes},
};
use log::warn;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct MBC3 {
    #[serde_as(as = "Box<[[_; ROM_BANK_SIZE]]>")]
    pub rom_banks: Box<[RomBank]>,
    #[serde_as(as = "Box<[[_; RAM_BANK_SIZE]]>")]
    pub ram_banks: Box<[RamBank]>,
    pub rom_bank_sel: u8,
    pub ram_bank_sel: u8,
    pub ram_enable: bool,
    pub rtc: Option<RTC>,
}

impl MBC3 {
    fn rom0(&self) -> &RomBank {
        &self.rom_banks[0]
    }

    fn rom1(&self) -> &RomBank {
        &self.rom_banks[self.rom_bank_sel as usize]
    }

    fn ram(&self) -> Option<&RamBank> {
        if self.ram_banks.is_empty() {
            return None;
        }
        Some(&self.ram_banks[self.ram_bank_sel as usize])
    }

    fn ram_mut(&mut self) -> Option<&mut RamBank> {
        if self.ram_banks.is_empty() {
            return None;
        }
        Some(&mut self.ram_banks[self.ram_bank_sel as usize])
    }

    fn set_ram_enable(&mut self, data: Word) {
        self.ram_enable = data == 0x0A;
    }

    fn set_rom_bank_sel(&mut self, data: Word) {
        let data = data & 0x7F;
        let data = if data == 0 { 1 } else { data };
        self.rom_bank_sel = data
    }

    fn set_ram_bank_sel(&mut self, data: Word) {
        self.ram_bank_sel = data;
    }
}

impl MBC for MBC3 {
    fn read(&self, addr: Addr) -> Word {
        match addr {
            ROM0_ADDR_LOW_BOUND..=ROM0_ADDR_HIGH_BOUND => {
                self.rom0()[(addr - ROM0_ADDR_LOW_BOUND) as usize]
            }
            ROM1_ADDR_LOW_BOUND..=ROM1_ADDR_HIGH_BOUND => {
                self.rom1()[(addr - ROM1_ADDR_LOW_BOUND) as usize]
            }
            RAM_ADDR_LOW_BOUND..=RAM_ADDR_HIGH_BOUND => match self.ram_bank_sel {
                0x00..=0x03 => {
                    if self.ram_enable {
                        if let Some(ram) = self.ram() {
                            ram[(addr - RAM_ADDR_LOW_BOUND) as usize]
                        } else {
                            0xFF
                        }
                    } else {
                        0xFF
                    }
                }
                0x08..=0x0C => match &self.rtc {
                    Some(rtc) => rtc.read(addr),
                    None => 0xFF,
                },
                _ => {
                    warn!(
                        "illegal read mbc3 cart at address: 0x{addr:04X} and ram_bank_sel: 0x{bank_sel}",
                        bank_sel = self.ram_bank_sel
                    );
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
            0x0000..=0x1FFF => self.set_ram_enable(data),
            0x2000..=0x3FFF => self.set_rom_bank_sel(data),
            0x4000..=0x5FFF => self.set_ram_bank_sel(data),
            0x6000..=0x7FFF => {
                if let Some(rtc) = &mut self.rtc {
                    rtc.set_latch(data)
                }
            }
            RAM_ADDR_LOW_BOUND..=RAM_ADDR_HIGH_BOUND => {
                if self.ram_bank_sel <= 0x03 {
                    if self.ram_enable {
                        if let Some(ram) = self.ram_mut() {
                            ram[(addr - RAM_ADDR_LOW_BOUND) as usize] = data;
                        }
                    }
                } else {
                    if let Some(rtc) = &mut self.rtc {
                        rtc.write(addr, data)
                    }
                }
            }
            _ => warn!("illegal write cart at address: 0x{addr:04X}"),
        }
    }

    fn cart_rom(&self) -> &Rom {
        let rom = self.rom_banks.as_ref();
        slice_as_bytes(rom)
    }

    fn new(rom: Box<[u8]>, ram_size: usize, has_rtc: bool, timestamp: i64) -> EmuResult<Self> {
        let rom_banks: Box<[RomBank]> = unsafe { bytes_to_slice(rom) };
        let ram_banks_num = ram_size / RAM_BANK_SIZE;
        let ram_banks = vec![[0; RAM_BANK_SIZE]; ram_banks_num].into_boxed_slice();
        let rtc = if has_rtc {
            Some(RTC::new(timestamp))
        } else {
            None
        };
        Ok(Self {
            rom_banks,
            ram_banks,
            rom_bank_sel: 1,
            ram_bank_sel: 0,
            ram_enable: false,
            rtc,
        })
    }
}
