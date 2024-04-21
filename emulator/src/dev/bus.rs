use crate::{
    cartridge::Cartridge,
    types::{Addr, Word},
};
use log::warn;

use super::{
    hram::HighRam, io_regs::IORegs, oam::ObjectAttributeMem, vram::VedioRam, wram::WorkRam,
};

/// ref https://gbdev.io/pandocs/Memory_Map.html
/// 0x0000 - 0x7FFF: 32KB CART ROM
/// 0x8000 - 0x9FFF: 8KB VRAM
/// 0XA000 - 0xBFFF: 8KB CART RAM
/// 0xC000 - 0xDFFF: 8kB WRAM
/// 0xE000 - 0xFDFF: FORBIDEN
/// 0xFE00 - 0xFE9F: Object Attribute Memory (OAM)
/// 0xFEA0 - 0xFEFF: FORBIDEN
/// 0xFF00 - 0xFF7F: I/O Registers
/// 0xFF80 - 0xFFFE: High Ram used as Func Call Stack
/// 0xFFFF - 0xFFFF: Interrupt Enable Register
/// 读取非法地址返回0xFF
/// 写操作非法地址不做任何操作
pub struct Bus {
    cartridge: Option<Cartridge>,
    vram: VedioRam,
    wram: WorkRam,
    oam: ObjectAttributeMem,
    io_regs: IORegs,
    hram: HighRam,
    ie_reg: Word,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            cartridge: None,
            vram: VedioRam::new(),
            wram: WorkRam::new(),
            oam: ObjectAttributeMem::new(),
            io_regs: IORegs::new(),
            hram: HighRam::new(),
            ie_reg: 0x00,
        }
    }

    pub fn read(&self, addr: Addr) -> Word {
        match addr {
            CART_ROM_LOW_BOUND..=CART_ROM_HIGH_BOUND_INCLUDED
            | CART_RAM_LOW_BOUND..=CART_RAM_HIGH_BOUND_INCLUDED => {
                if let Some(ref c) = self.cartridge {
                    c.read(addr)
                } else {
                    warn!("no cartridge is plugged in! illegal read at address: 0x:{addr:04X}");
                    0x00
                }
            }
            VRAM_LOW_BOUND..=VRAM_HIGH_BOUND_INCLUDED => self.vram.read(addr),
            WRAM_LOW_BOUND..=WRAM_HIGH_BOUND_INCLUDED => self.wram.read(addr),
            OAM_LOW_BOUND..=OAM_HIGH_BOUND_INCLUDED => self.oam.read(addr),
            IO_LOW_BOUND..=IO_HIGH_BOUND_INCLUDED => self.io_regs.read(addr),
            HRAM_LOW_BOUND..=HRAM_HIGH_BOUND_INCLUDED => self.hram.read(addr),
            IE_REG_ADDR => self.ie_reg,
            _ => {
                warn!("illegal read at address: 0x{addr:04X}");
                0xFF
            }
        }
    }

    pub fn write(&mut self, addr: Addr, data: Word) {
        match addr {
            CART_ROM_LOW_BOUND..=CART_ROM_HIGH_BOUND_INCLUDED
            | CART_RAM_LOW_BOUND..=CART_RAM_HIGH_BOUND_INCLUDED => {
                if let Some(ref mut c) = self.cartridge {
                    c.write(addr, data);
                } else {
                    warn!("no cartridge is plugged in! illegal write at address: 0x:{addr:04X}");
                }
            }
            VRAM_LOW_BOUND..=VRAM_HIGH_BOUND_INCLUDED => self.vram.write(addr, data),
            WRAM_LOW_BOUND..=WRAM_HIGH_BOUND_INCLUDED => self.wram.write(addr, data),
            OAM_LOW_BOUND..=OAM_HIGH_BOUND_INCLUDED => self.oam.write(addr, data),
            HRAM_LOW_BOUND..=HRAM_HIGH_BOUND_INCLUDED => self.hram.write(addr, data),
            IE_REG_ADDR => self.ie_reg = data,
            _ => warn!("illegal write at address: 0x{addr:04X}"),
        }
    }
}

pub const CART_ROM_LOW_BOUND: Addr = 0x0000;
pub const VRAM_LOW_BOUND: Addr = 0x8000;
pub const CART_RAM_LOW_BOUND: Addr = 0xA000;
pub const WRAM_LOW_BOUND: Addr = 0xC000;
pub const OAM_LOW_BOUND: Addr = 0xFE00;
pub const IO_LOW_BOUND: Addr = 0xFF00;
pub const HRAM_LOW_BOUND: Addr = 0xFF80;
pub const IE_REG_ADDR: Addr = 0xFFFF;

pub const CART_ROM_HIGH_BOUND: Addr = VRAM_LOW_BOUND;
pub const VRAM_HIGH_BOUND: Addr = CART_RAM_LOW_BOUND;
pub const CART_RAM_HIGH_BOUND: Addr = WRAM_LOW_BOUND;
pub const WRAM_HIGH_BOUND: Addr = 0xE000;
pub const OAM_HIGH_BOUND: Addr = 0xFEA0;
pub const IO_HIGH_BOUND: Addr = HRAM_LOW_BOUND;
pub const HRAM_HIGH_BOUND: Addr = IE_REG_ADDR;

pub const CART_ROM_SIZE: usize = (CART_ROM_HIGH_BOUND - CART_ROM_LOW_BOUND) as usize;
pub const VRAM_SIZE: usize = (VRAM_HIGH_BOUND - VRAM_LOW_BOUND) as usize;
pub const CART_RAM_SIZE: usize = (CART_RAM_HIGH_BOUND - CART_RAM_LOW_BOUND) as usize;
pub const WRAM_SIZE: usize = (WRAM_HIGH_BOUND - WRAM_LOW_BOUND) as usize;
pub const OAM_SIZE: usize = (OAM_HIGH_BOUND - OAM_LOW_BOUND) as usize;
pub const IO_SIZE: usize = (IO_HIGH_BOUND - IO_LOW_BOUND) as usize;
pub const HRAM_SIZE: usize = (HRAM_HIGH_BOUND - HRAM_LOW_BOUND) as usize;

pub const CART_ROM_HIGH_BOUND_INCLUDED: Addr = CART_ROM_HIGH_BOUND - 1;
pub const VRAM_HIGH_BOUND_INCLUDED: Addr = VRAM_HIGH_BOUND - 1;
pub const CART_RAM_HIGH_BOUND_INCLUDED: Addr = CART_RAM_HIGH_BOUND - 1;
pub const WRAM_HIGH_BOUND_INCLUDED: Addr = WRAM_HIGH_BOUND - 1;
pub const OAM_HIGH_BOUND_INCLUDED: Addr = OAM_HIGH_BOUND - 1;
pub const IO_HIGH_BOUND_INCLUDED: Addr = IO_HIGH_BOUND - 1;
pub const HRAM_HIGH_BOUND_INCLUDED: Addr = HRAM_HIGH_BOUND - 1;

pub trait BusDevice {
    /// 默认返回0xFF
    fn read(&self, addr: Addr) -> Word {
        warn!("illegal read at address: 0x{addr:04X}");
        0xFF
    }

    #[allow(unused)]
    fn write(&mut self, addr: Addr, data: Word) {
        warn!("illegal write at address: 0x{addr:04X}");
    }
}
