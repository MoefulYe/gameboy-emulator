use self::header::Header;
use crate::{
    dev::BusDevice,
    types::{Addr, Word},
};
use log::warn;

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

    pub fn check_and_get_info(&self) -> PluginCartResult {
        let header = self.header();
        if let Some(info) = header
            .check_logo()
            .or_else(|| header.checksum())
            .map(|e| e.info())
        {
            PluginCartResult::Err { info }
        } else {
            let info = header.info();
            PluginCartResult::Ok { info }
        }
    }
}

impl BusDevice for Cartridge {
    fn read(&self, addr: Addr) -> Word {
        // https://gbdev.io/pandocs/Memory_Map.html
        match addr {
            0x0000..=0x3FFF => unsafe { *self.rom.get_unchecked(addr as usize) },
            0x4000..=0x7FFF => todo!(),
            0xA000..=0xBFFF => todo!(),
            _ => {
                warn!("illegal read from cartridge at address: {addr:04X}");
                0xFF
            }
        }
    }

    fn write(&mut self, addr: Addr, data: Word) {
        match addr {
            0xA000..=0xBFFF => todo!(),
            _ => warn!("illegal write to cartridge at address: {addr:04X}"),
        }
    }
}

#[allow(non_snake_case)]
mod tsify_derive {
    use serde::Serialize;
    use tsify::Tsify;

    use crate::error::EmulatorErrorInfo;

    #[derive(Serialize, Tsify, Debug)]
    #[tsify(into_wasm_abi)]
    #[serde(rename_all = "camelCase")]
    pub struct CartridgeInfo {
        pub title: Box<str>,
        pub cart_type: &'static str,
        pub rom_size: usize,
        pub ram_size: Option<usize>,
        pub dest: &'static str,
        pub publisher: &'static str,
        pub version: u8,
    }

    #[derive(Serialize, Tsify)]
    #[tsify(into_wasm_abi)]
    #[serde(tag = "status")]
    pub enum PluginCartResult {
        #[serde(rename = "ok")]
        Ok { info: CartridgeInfo },
        #[serde(rename = "error")]
        Err { info: Box<EmulatorErrorInfo> },
    }
}

pub use tsify_derive::{CartridgeInfo, PluginCartResult};
