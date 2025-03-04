use super::{CartInfo, Rom};
use crate::error::{
    EmuErr, EmuResult, EmulatorError, InvalidChecksum, InvalidLogo, InvalidRomSize,
};
use core::mem::offset_of;
use std::mem::size_of;

const KB: usize = 1024;
const ENTRY_SIZE: usize = 0x04;
const LOGO_SIZE: usize = 0x30;
const TITLE_SIZE: usize = 0x10;
const ROM_OFFSET: usize = 0x0100;

/// https://gbdev.io/pandocs/The_Cartridge_Header.html
/// mapped to 0x0100-0x014F in ROM
#[repr(C)]
pub struct Header {
    /// 卡带加载后的首先被执行的指令
    #[allow(unused)]
    entry: [u8; ENTRY_SIZE],
    /// 任天堂公司标志，固定值
    nintendo_logo: [u8; LOGO_SIZE],
    /// UPPER CASE ASCII, padding with '\0'
    /// GBC
    title: [u8; TITLE_SIZE],
    new_lic_code: [u8; 2],
    /// 本模拟器不是实现SGB功能，该字段忽略
    #[allow(unused)]
    sgb_flag: u8,
    cart_type: u8,
    rom_size: u8,
    ram_size: u8,
    dest_code: u8,
    old_lic_code: u8,
    version: u8,
    checksum: u8,
    /// 模拟器内部不会校验, 需要游戏程序自行校验
    #[allow(unused)]
    global_checksum: [u8; 2],
}

pub enum MBCType {
    NoMBC,
    MBC1,
    MBC2,
    MBC3,
}

impl Header {
    pub unsafe fn from_rom_unchecked<'a>(rom: &'a Rom) -> &'a Self {
        unsafe {
            let base = rom.as_ptr().add(ROM_OFFSET) as *const Self;
            &*base
        }
    }

    pub fn from_rom(rom: &Rom) -> EmuResult<&Self> {
        if rom.len() < size_of::<Header>() + ROM_OFFSET {
            return EmuErr(InvalidRomSize { size: rom.len() });
        }
        let header = unsafe { Self::from_rom_unchecked(rom) };
        if let Some(err) = header.check_logo() {
            EmuErr(err)
        } else if let Some(err) = header.checksum() {
            EmuErr(err)
        } else {
            Ok(header)
        }
    }

    //TODO The CGB and later models only check the top half of the logo (the first $18 bytes).
    pub fn check_logo(&self) -> Option<EmulatorError> {
        const CORRECT_LOGO: &[u8; LOGO_SIZE] = &[
            0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C,
            0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
            0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
            0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
        ];
        if &self.nintendo_logo == CORRECT_LOGO {
            None
        } else {
            Some(InvalidLogo {
                expected: CORRECT_LOGO,
                actual: self.nintendo_logo,
            })
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
        const ROM_BASE_SIZE: usize = 32 * KB;
        ROM_BASE_SIZE << self.rom_size
    }

    pub fn ram_size(&self) -> usize {
        match self.ram_size {
            0x00 => 0,
            0x02 => 8 * KB,
            0x03 => 32 * KB,
            0x04 => 128 * KB,
            0x05 => 64 * KB,
            _ => 0,
        }
    }

    pub fn dest(&self) -> &'static str {
        match self.dest_code {
            0x00 => "Japan",
            0x01 => "Overseas",
            _ => "Unknown",
        }
    }

    pub fn checksum(&self) -> Option<EmulatorError> {
        const OFFSET_OF_TITLE: usize = offset_of!(Header, title);
        const OFFSET_OF_CHECKSUM: usize = offset_of!(Header, checksum);
        let to_check = unsafe {
            core::slice::from_raw_parts(
                &self.title as *const _ as usize as *const u8,
                OFFSET_OF_CHECKSUM - OFFSET_OF_TITLE,
            )
        };
        let sum = to_check
            .iter()
            .fold(0u8, |acc, &i| acc.wrapping_sub(i).wrapping_sub(1));
        if sum == self.checksum {
            None
        } else {
            Some(InvalidChecksum {
                expected: sum,
                actual: self.checksum,
            })
        }
    }

    pub fn cart_typename(&self) -> &'static str {
        match self.cart_type {
            0x00 => "ROM ONLY",
            0x01 => "MBC1",
            0x02 => "MBC1+RAM",
            0x03 => "MBC1+RAM+BATTERY",
            0x05 => "MBC2",
            0x06 => "MBC2+BATTERY",
            0x08 => "ROM+RAM",
            0x09 => "ROM+RAM+BATTERY",
            0x0B => "MMM01",
            0x0C => "MMM01+RAM",
            0x0D => "MMM01+RAM+BATTERY",
            0x0F => "MBC3+TIMER+BATTERY",
            0x10 => "MBC3+TIMER+RAM+BATTERY",
            0x11 => "MBC3",
            0x12 => "MBC3+RAM",
            0x13 => "MBC3+RAM+BATTERY",
            0x19 => "MBC5",
            0x1A => "MBC5+RAM",
            0x1B => "MBC5+RAM+BATTERY",
            0x1C => "MBC5+RUMBLE",
            0x1D => "MBC5+RUMBLE+RAM",
            0x1E => "MBC5+RUMBLE+RAM+BATTERY",
            0x20 => "MBC6",
            0x22 => "MBC7+SENSOR+RUMBLE+RAM+BATTERY",
            0xFC => "POCKET CAMERA",
            0xFD => "BANDAI TAMA5",
            0xFE => "HuC3",
            0xFF => "HuC1+RAM+BATTERY",
            _ => "UNKNOWN",
        }
    }

    pub fn mbc_type(&self) -> Option<MBCType> {
        use MBCType::*;
        match self.cart_type {
            0x00 | 0x08 | 0x09 => Some(NoMBC),
            0x01 | 0x02 | 0x03 => Some(MBC1),
            0x05 | 0x06 => Some(MBC2),
            0x0F | 0x10 | 0x11 | 0x12 | 0x13 => Some(MBC3),
            _ => None,
        }
    }

    pub fn has_rtc(&self) -> bool {
        match self.cart_type {
            0x0F | 0x10 => true,
            _ => false,
        }
    }

    pub fn has_battery(&self) -> bool {
        match self.cart_type {
            3 | 6 | 9 | 13 | 15 | 16 | 19 | 27 | 30 | 34 => true,
            _ => false,
        }
    }

    pub fn publisher(&self) -> &'static str {
        if let Some(publisher) = self.old_publisher() {
            publisher
        } else if let Some(publisher) = self.new_publisher() {
            publisher
        } else {
            "UNKNOWN"
        }
    }

    pub fn version(&self) -> u8 {
        self.version
    }

    pub fn info(&self) -> CartInfo {
        let title = self.title().to_string();
        let cart_type = self.cart_typename();
        let rom_size = self.rom_size();
        let ram_size = self.ram_size();
        let dest = self.dest();
        let publisher = self.publisher();
        let version = self.version();
        CartInfo {
            title,
            cart_type,
            rom_size,
            ram_size,
            dest,
            publisher,
            version,
        }
    }

    fn new_publisher(&self) -> Option<&'static str> {
        let [a, b] = self.new_lic_code;
        let code = (b as u16) << 8 | a as u16;
        match code {
            0x00 => Some("None"),
            0x01 => Some("Nintendo R&D1"),
            0x08 => Some("Capcom"),
            0x13 => Some("Electronic Arts"),
            0x18 => Some("Hudson Soft"),
            0x19 => Some("b-ai"),
            0x20 => Some("kss"),
            0x22 => Some("pow"),
            0x24 => Some("PCM Complete"),
            0x25 => Some("san-x"),
            0x28 => Some("Kemco Japan"),
            0x29 => Some("seta"),
            0x30 => Some("Viacom"),
            0x31 => Some("Nintendo"),
            0x32 => Some("Bandai"),
            0x33 => Some("Ocean/Acclaim"),
            0x34 => Some("Konami"),
            0x35 => Some("Hector"),
            0x37 => Some("Taito"),
            0x38 => Some("Hudson"),
            0x39 => Some("Banpresto"),
            0x41 => Some("Ubi Soft"),
            0x42 => Some("Atlus"),
            0x44 => Some("Malibu"),
            0x46 => Some("angel"),
            0x47 => Some("Bullet-Proof"),
            0x49 => Some("irem"),
            0x50 => Some("Absolute"),
            0x51 => Some("Acclaim"),
            0x52 => Some("Activision"),
            0x53 => Some("American sammy"),
            0x54 => Some("Konami"),
            0x55 => Some("Hi tech entertainment"),
            0x56 => Some("LJN"),
            0x57 => Some("Matchbox"),
            0x58 => Some("Mattel"),
            0x59 => Some("Milton Bradley"),
            0x60 => Some("Titus"),
            0x61 => Some("Virgin"),
            0x64 => Some("LucasArts"),
            0x67 => Some("Ocean"),
            0x69 => Some("Electronic Arts"),
            0x70 => Some("Infogrames"),
            0x71 => Some("Interplay"),
            0x72 => Some("Broderbund"),
            0x73 => Some("sculptured"),
            0x75 => Some("sci"),
            0x78 => Some("THQ"),
            0x79 => Some("Accolade"),
            0x80 => Some("misawa"),
            0x83 => Some("lozc"),
            0x86 => Some("Tokuma Shoten Intermedia"),
            0x87 => Some("Tsukuda Original"),
            0x91 => Some("Chunsoft"),
            0x92 => Some("Video system"),
            0x93 => Some("Ocean/Acclaim"),
            0x95 => Some("Varie"),
            0x96 => Some("Yonezawa/s’pal"),
            0x97 => Some("Kaneko"),
            0x99 => Some("Pack in soft"),
            0xA4 => Some("Konami (Yu-Gi-Oh!)"),
            _ => None,
        }
    }

    fn old_publisher(&self) -> Option<&'static str> {
        match self.old_lic_code {
            0x00 => Some("None"),
            0x01 => Some("Nintendo"),
            0x08 => Some("Capcom"),
            0x09 => Some("Hot-B"),
            0x0A => Some("Jaleco"),
            0x0B => Some("Coconuts Japan"),
            0x0C => Some("Elite Systems"),
            0x13 => Some("EA (Electronic Arts)"),
            0x18 => Some("Hudsonsoft"),
            0x19 => Some("ITC Entertainment"),
            0x1A => Some("Yanoman"),
            0x1D => Some("Japan Clary"),
            0x1F => Some("Virgin Interactive"),
            0x24 => Some("PCM Complete"),
            0x25 => Some("San-X"),
            0x28 => Some("Kotobuki Systems"),
            0x29 => Some("Seta"),
            0x30 => Some("Infogrames"),
            0x31 => Some("Nintendo"),
            0x32 => Some("Bandai"),
            0x33 => None,
            0x34 => Some("Konami"),
            0x35 => Some("HectorSoft"),
            0x38 => Some("Capcom"),
            0x39 => Some("Banpresto"),
            0x3C => Some(".Entertainment i"),
            0x3E => Some("Gremlin"),
            0x41 => Some("Ubisoft"),
            0x42 => Some("Atlus"),
            0x44 => Some("Malibu"),
            0x46 => Some("Angel"),
            0x47 => Some("Spectrum Holoby"),
            0x49 => Some("Irem"),
            0x4A => Some("Virgin Interactive"),
            0x4D => Some("Malibu"),
            0x4F => Some("U.S. Gold"),
            0x50 => Some("Absolute"),
            0x51 => Some("Acclaim"),
            0x52 => Some("Activision"),
            0x53 => Some("American Sammy"),
            0x54 => Some("GameTek"),
            0x55 => Some("Park Place"),
            0x56 => Some("LJN"),
            0x57 => Some("Matchbox"),
            0x59 => Some("Milton Bradley"),
            0x5A => Some("Mindscape"),
            0x5B => Some("Romstar"),
            0x5C => Some("Naxat Soft"),
            0x5D => Some("Tradewest"),
            0x60 => Some("Titus"),
            0x61 => Some("Virgin Interactive"),
            0x67 => Some("Ocean Interactive"),
            0x69 => Some("EA (Electronic Arts)"),
            0x6E => Some("Elite Systems"),
            0x6F => Some("Electro Brain"),
            0x70 => Some("Infogrames"),
            0x71 => Some("Interplay"),
            0x72 => Some("Broderbund"),
            0x73 => Some("Sculptered Soft"),
            0x75 => Some("The Sales Curve"),
            0x78 => Some("t.hq"),
            0x79 => Some("Accolade"),
            0x7A => Some("Triffix Entertainment"),
            0x7C => Some("Microprose"),
            0x7F => Some("Kemco"),
            0x80 => Some("Misawa Entertainment"),
            0x83 => Some("Lozc"),
            0x86 => Some("Tokuma Shoten Intermedia"),
            0x8B => Some("Bullet-Proof Software"),
            0x8C => Some("Vic Tokai"),
            0x8E => Some("Ape"),
            0x8F => Some("I’Max"),
            0x91 => Some("Chunsoft Co."),
            0x92 => Some("Video System"),
            0x93 => Some("Tsubaraya Productions Co."),
            0x95 => Some("Varie Corporation"),
            0x96 => Some("Yonezawa/S’Pal"),
            0x97 => Some("Kaneko"),
            0x99 => Some("Arc"),
            0x9A => Some("Nihon Bussan"),
            0x9B => Some("Tecmo"),
            0x9C => Some("Imagineer"),
            0x9D => Some("Banpresto"),
            0x9F => Some("Nova"),
            0xA1 => Some("Hori Electric"),
            0xA2 => Some("Bandai"),
            0xA4 => Some("Konami"),
            0xA6 => Some("Kawada"),
            0xA7 => Some("Takara"),
            0xA9 => Some("Technos Japan"),
            0xAA => Some("Broderbund"),
            0xAC => Some("Toei Animation"),
            0xAD => Some("Toho"),
            0xAF => Some("Namco"),
            0xB0 => Some("acclaim"),
            0xB1 => Some("ASCII or Nexsoft"),
            0xB2 => Some("Bandai"),
            0xB4 => Some("Square Enix"),
            0xB6 => Some("HAL Laboratory"),
            0xB7 => Some("SNK"),
            0xB9 => Some("Pony Canyon"),
            0xBA => Some("Culture Brain"),
            0xBB => Some("Sunsoft"),
            0xBD => Some("Sony Imagesoft"),
            0xBF => Some("Sammy"),
            0xC0 => Some("Taito"),
            0xC2 => Some("Kemco"),
            0xC3 => Some("Squaresoft"),
            0xC4 => Some("Tokuma Shoten Intermedia"),
            0xC5 => Some("Data East"),
            0xC6 => Some("Tonkinhouse"),
            0xC8 => Some("Koei"),
            0xC9 => Some("UFL"),
            0xCA => Some("Ultra"),
            0xCB => Some("Vap"),
            0xCC => Some("Use Corporation"),
            0xCD => Some("Meldac"),
            0xCE => Some(".Pony Canyon or"),
            0xCF => Some("Angel"),
            0xD0 => Some("Taito"),
            0xD1 => Some("Sofel"),
            0xD2 => Some("Quest"),
            0xD3 => Some("Sigma Enterprises"),
            0xD4 => Some("ASK Kodansha Co."),
            0xD6 => Some("Naxat Soft"),
            0xD7 => Some("Copya System"),
            0xD9 => Some("Banpresto"),
            0xDA => Some("Tomy"),
            0xDB => Some("LJN"),
            0xDD => Some("NCS"),
            0xDE => Some("Human"),
            0xDF => Some("Altron"),
            0xE0 => Some("Jaleco"),
            0xE1 => Some("Towa Chiki"),
            0xE2 => Some("Yutaka"),
            0xE3 => Some("Varie"),
            0xE5 => Some("Epcoh"),
            0xE7 => Some("Athena"),
            0xE8 => Some("Asmik ACE Entertainment"),
            0xE9 => Some("Natsume"),
            0xEA => Some("King Records"),
            0xEB => Some("Atlus"),
            0xEC => Some("Epic/Sony Records"),
            0xEE => Some("IGS"),
            0xF0 => Some("A Wave"),
            0xF3 => Some("Extreme Entertainment"),
            0xFF => Some("LJN"),
            _ => Some("UNKNOWN"),
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    const ROMS_PATH: &'static str = "../public/roms";
    fn read_roms() -> Vec<Vec<u8>> {
        fs::read_dir(ROMS_PATH)
            .unwrap()
            .map(|entry| fs::read(entry.unwrap().path()).unwrap())
            .collect()
    }

    #[test]
    fn test() {
        use super::Header;

        println!("reading roms from {ROMS_PATH}");
        let roms = read_roms();
        for (i, rom) in roms.iter().enumerate() {
            println!("testing rom {}", i + 1);
            let header = unsafe { Header::from_rom_unchecked(&rom) };
            let title = header.title();
            let ty = header.cart_typename();
            let rom_size = header.rom_size();
            let ram_size = header.ram_size();
            let publisher = header.publisher();
            let version = header.version();
            let dest = header.dest();
            println!("title={title},\ntype={ty},\nrom_size={rom_size},\nram_size={ram_size},\npublisher={publisher},\nversion={version}\ndest={dest}\n------------------------------");
        }
    }
}
