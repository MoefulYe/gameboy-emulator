use log::warn;

use crate::{
    dev::BusDevice,
    error::Result,
    types::{Addr, Word},
};

use self::header::Header;

mod header;

pub type Rom = [u8];
pub type Ram = [u8];
pub struct Cartridge {
    rom: Box<Rom>,
    ram: Option<Box<Ram>>,
}

impl Cartridge {
    pub fn new(rom: Box<Rom>) -> Self {
        let ram = Header::from_rom(&rom).ram_size().and_then(|s| {
            if s == 0 {
                None
            } else {
                Some(vec![0; s].into_boxed_slice())
            }
        });
        Cartridge { rom, ram }
    }

    pub fn header<'a>(&'a self) -> &'a Header {
        Header::from_rom(&self.rom)
    }
}

impl BusDevice for Cartridge {
    fn read(&self, addr: Addr) -> Result<Word> {
        // https://gbdev.io/pandocs/Memory_Map.html
        match addr {
            0x0000..=0x3FFF => Ok(unsafe { *self.rom.get_unchecked(addr as usize) }),
            0x4000..=0x7FFF => todo!(),
            0xA000..=0xBFFF => todo!(),
            _ => {
                warn!("illegal read from cartridge at address: {addr:04X}");
                Ok(0xFF)
            }
        }
    }

    fn write(&mut self, addr: Addr, data: Word) -> Result {
        match addr {
            0xA000..=0xBFFF => todo!(),
            _ => {
                warn!("illegal write to cartridge at address: {addr:04X}");
                Ok(())
            }
        }
    }
}
