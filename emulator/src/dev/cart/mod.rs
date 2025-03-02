use self::header::Header;
use crate::{
    dev::MemoryRegion,
    error::{EmuErr, EmuResult, EmulatorError::UnknownMBCType},
    types::{Addr, Word},
};
use header::MBCType;
use mbc::{mbc1::MBC1, mbc2::MBC2, mbc3::MBC3, no_mbc::NoMBC, MBC};
use serde::{Deserialize, Serialize};

mod header;
mod mbc;

const ROM0_ADDR_LOW_BOUND: Addr = 0x0000;
const ROM0_ADDR_HIGH_BOUND: Addr = 0x3FFF;
const ROM1_ADDR_LOW_BOUND: Addr = 0x4000;
const ROM1_ADDR_HIGH_BOUND: Addr = 0x7FFF;
const RAM_ADDR_LOW_BOUND: Addr = 0xA000;
const RAM_ADDR_HIGH_BOUND: Addr = 0xBFFF;

pub type Rom = [u8];

#[derive(Serialize, Deserialize)]
pub enum Cart {
    NoMBC(NoMBC),
    MBC1(MBC1),
    MBC2(MBC2),
    MBC3(MBC3),
}

impl Cart {
    pub fn new(rom: Box<Rom>, timestamp: i64) -> EmuResult<Self> {
        let (ram_size, mbc_type, has_rtc) = {
            let header = Header::from_rom(&rom)?;
            let mbc_type = header.mbc_type();
            let has_rtc = header.has_rtc();
            let ram_size = header.ram_size();
            (ram_size, mbc_type, has_rtc)
        };
        match mbc_type {
            Some(MBCType::NoMBC) => Ok(Cart::NoMBC(NoMBC::new(rom, ram_size, has_rtc, timestamp)?)),
            Some(MBCType::MBC1) => Ok(Cart::MBC1(MBC1::new(rom, ram_size, has_rtc, timestamp)?)),
            Some(MBCType::MBC2) => Ok(Cart::MBC2(MBC2::new(rom, ram_size, has_rtc, timestamp)?)),
            Some(MBCType::MBC3) => Ok(Cart::MBC3(MBC3::new(rom, ram_size, has_rtc, timestamp)?)),
            None => EmuErr(UnknownMBCType),
        }
    }

    pub fn rom(&self) -> &Rom {
        match self {
            Cart::NoMBC(c) => c.cart_rom(),
            Cart::MBC1(c) => c.cart_rom(),
            Cart::MBC2(c) => c.cart_rom(),
            Cart::MBC3(c) => c.cart_rom(),
        }
    }

    pub fn header(&self) -> &Header {
        unsafe { Header::from_rom_unchecked(self.rom()) }
    }

    pub fn update_rtc(&mut self, timestamp: i64) {
        if let Cart::MBC3(MBC3 { rtc: Some(rtc), .. }) = self {
            rtc.update(timestamp)
        }
    }
}

impl MemoryRegion for Cart {
    fn read(&self, addr: Addr) -> Word {
        match self {
            Cart::NoMBC(c) => c.read(addr),
            Cart::MBC1(c) => c.read(addr),
            Cart::MBC2(c) => c.read(addr),
            Cart::MBC3(c) => c.read(addr),
        }
    }

    fn write(&mut self, addr: Addr, data: Word) {
        match self {
            Cart::NoMBC(c) => c.write(addr, data),
            Cart::MBC1(c) => c.write(addr, data),
            Cart::MBC2(c) => c.write(addr, data),
            Cart::MBC3(c) => c.write(addr, data),
        }
    }
}

#[allow(non_snake_case)]
mod tsify_derive {
    use serde::Serialize;
    use tsify::Tsify;

    use crate::error::EmuResult;

    #[derive(Serialize, Tsify, Debug)]
    #[tsify(into_wasm_abi)]
    #[serde(rename_all = "camelCase")]
    pub struct CartInfo {
        pub title: String,
        pub cart_type: &'static str,
        pub rom_size: usize,
        pub ram_size: usize,
        pub dest: &'static str,
        pub publisher: &'static str,
        pub version: u8,
    }

    #[derive(Serialize, Tsify)]
    #[tsify(into_wasm_abi)]
    #[serde(tag = "status")]
    pub enum LoadCartResult {
        #[serde(rename = "ok")]
        Ok { info: CartInfo },
        #[serde(rename = "error")]
        Err { msg: String },
    }

    impl From<EmuResult<CartInfo>> for LoadCartResult {
        fn from(value: EmuResult<CartInfo>) -> Self {
            match value {
                Ok(info) => LoadCartResult::Ok { info },
                Err(err) => LoadCartResult::Err { msg: err.msg() },
            }
        }
    }
}

pub use tsify_derive::{CartInfo, LoadCartResult};
