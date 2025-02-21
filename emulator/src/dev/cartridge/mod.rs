use self::header::Header;
use crate::{
    dev::BusDevice,
    types::{Addr, Word},
};
use log::warn;
use mbc::{mbc1::MBC1, mbc2::MBC2, mbc3::MBC3, no_mbc::NoMBC, MBC};
use std::mem::size_of;

mod header;
mod mbc;

const ROM0_ADDR_LOW_BOUND: Addr = 0x0000;
const ROM0_ADDR_HIGH_BOUND: Addr = 0x3FFF;
const ROM1_ADDR_LOW_BOUND: Addr = 0x4000;
const ROM1_ADDR_HIGH_BOUND: Addr = 0x7FFF;
const RAM_ADDR_LOW_BOUND: Addr = 0xA000;
const RAM_ADDR_HIGH_BOUND: Addr = 0xBFFF;

pub type Rom = [u8];
pub enum Cartridge {
    NoMBC(NoMBC),
    MBC1(MBC1),
    MBC2(MBC2),
    MBC3(MBC3),
}

impl Cartridge {
    pub fn new(rom: Box<Rom>) -> Option<Self> {
        todo!()
    }

    fn header<'a>(&'a self) -> &'a Header {
        unsafe { Header::from_rom(&self.rom) }
    }

    pub fn validate(&self) -> Result<CartridgeInfo, String> {
        let header = self.header();
        match header.validate() {
            Some(msg) => Err(msg),
            None => Ok(header.info()),
        }
    }

    pub fn validate_header(rom: &Rom) -> Result<CartridgeInfo, String> {
        if rom.len() < size_of::<Header>() {
            return Err("illegal size".to_owned());
        }
        let header = unsafe { Header::from_rom(rom) };
        match header.validate() {
            Some(msg) => Err(msg),
            None => Ok(header.info()),
        }
    }
}

impl BusDevice for Cartridge {
    fn read(&self, addr: Addr) -> Word {
        match self {
            Cartridge::NoMBC(c) => c.read(addr),
            Cartridge::MBC1(c) => c.read(addr),
            Cartridge::MBC2(c) => c.read(addr),
            Cartridge::MBC3(c) => c.read(addr),
        }
    }

    fn write(&mut self, addr: Addr, data: Word) {
        match self {
            Cartridge::NoMBC(c) => c.write(addr, data),
            Cartridge::MBC1(c) => c.write(addr, data),
            Cartridge::MBC2(c) => c.write(addr, data),
            Cartridge::MBC3(c) => c.write(addr, data),
        }
    }
}

#[allow(non_snake_case)]
mod tsify_derive {
    use serde::Serialize;
    use tsify::Tsify;

    #[derive(Serialize, Tsify, Debug)]
    #[tsify(into_wasm_abi)]
    #[serde(rename_all = "camelCase")]
    pub struct CartridgeInfo {
        pub title: String,
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
    pub enum LoadRomResult {
        #[serde(rename = "ok")]
        Ok { info: CartridgeInfo },
        #[serde(rename = "error")]
        Err { msg: String },
    }

    impl From<Result<CartridgeInfo, String>> for LoadRomResult {
        fn from(value: Result<CartridgeInfo, String>) -> Self {
            match value {
                Ok(info) => Self::Ok { info },
                Err(msg) => Self::Err { msg },
            }
        }
    }
}

pub use tsify_derive::{CartridgeInfo, LoadRomResult};
