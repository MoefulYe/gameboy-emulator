use super::Rom;
use crate::error::{EmulatorError, Result};
use core::mem::offset_of;

const KB: usize = 1024;
const MB: usize = 1024 * KB;
const ENTRY_SIZE: usize = 0x04;
const LOGO_SIZE: usize = 0x30;
const CORRECT_LOGO: &[u8; LOGO_SIZE] = &[
    0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
    0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
    0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
];

const TITLE_SIZE: usize = 0x10;
const ROM_OFFSET: usize = 0x0100;
const ROM_SIZ_BASE: usize = 32 * KB;

/// https://gbdev.io/pandocs/The_Cartridge_Header.html
/// mapped to 0x0100-0x014F in ROM
#[repr(C)]
pub struct Header {
    /// 卡带加载后的首先被执行的指令
    pub entry: [u8; ENTRY_SIZE],
    /// 任天堂公司标志，固定值
    pub nintendo_logo: [u8; LOGO_SIZE],
    /// UPPER CASE ASCII, padding with '\0'
    pub title: [u8; TITLE_SIZE],
    pub new_lic_code: [u8; 2],
    /// 本模拟器不是实现SGB功能，该字段忽略
    #[allow(unused)]
    pub sgb_flag: u8,
    pub cart_type: u8,
    pub rom_size: u8,
    pub ram_size: u8,
    pub dest_code: u8,
    pub old_lic_code: u8,
    pub version: u8,
    pub checksum: u8,
    /// 模拟器内部不会校验, 需要游戏程序自行校验
    #[allow(unused)]
    pub global_checksum: [u8; 2],
}

impl Header {
    pub fn from_rom<'a>(rom: &'a Rom) -> &'a Self {
        unsafe {
            let base = rom.as_ptr().add(ROM_OFFSET) as *const Self;
            &*base
        }
    }

    //TODO The CGB and later models only check the top half of the logo (the first $18 bytes).
    pub fn check_logo(&self) -> Result {
        if &self.nintendo_logo == CORRECT_LOGO {
            Ok(())
        } else {
            Err(EmulatorError::InvalidLogo)
        }
    }

    pub fn title<'a>(&'a self) -> &'a str {
        let end = self
            .title
            .iter()
            .enumerate()
            .find_map(|(i, &c)| if c == 0 { Some(i) } else { None })
            .unwrap_or(TITLE_SIZE);
        unsafe { core::str::from_utf8_unchecked(&self.title[..end]) }
    }

    pub fn rom_size(&self) -> usize {
        ROM_SIZ_BASE << self.rom_size
    }

    pub fn ram_size(&self) -> Result<usize> {
        match self.ram_size {
            0x00 => Ok(0),
            0x02 => Ok(8 * KB),
            0x03 => Ok(32 * KB),
            0x04 => Ok(128 * KB),
            0x05 => Ok(64 * KB),
            _ => Err(EmulatorError::InvalidRamSize),
        }
    }

    pub fn dest(&self) -> Dest {
        match self.dest_code {
            0 => Dest::Japan,
            1 => Dest::Overseas,
            _ => Dest::Unknown,
        }
    }

    pub fn checksum(&self) -> Result {
        const OFFSET_OF_VERSION: usize = offset_of!(Header, version);
        const OFFSET_OF_CHECKSUM: usize = offset_of!(Header, checksum);
        let to_check = unsafe {
            core::slice::from_raw_parts(
                self as *const _ as u8 as *const u8,
                OFFSET_OF_CHECKSUM - OFFSET_OF_VERSION,
            )
        };
        let sum = to_check.iter().fold(0u8, |acc, &i| acc - i - 1);
        if sum == self.checksum {
            Ok(())
        } else {
            Err(EmulatorError::InvalidChecksum)
        }
    }
}

pub enum Dest {
    Japan = 0,
    Overseas = 1,
    Unknown = 2,
}

pub enum CartType {
    RomOnly = 0x00,
    /// 有mbc但没有ram
    MBC1 = 0x01,
    /// 有mbc和ram
    MBC1Ram = 0x02,
    /// 有mbc和ram和电池
    MBC1RamBattery = 0x03,
}
