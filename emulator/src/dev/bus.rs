use crate::{
    cartridge::Cartridge,
    error::{EmulatorError, Result},
    types::{Addr, Word},
};
use log::warn;

use super::{
    int_regs::{InterruptEnableRegsiter, InterruptMaskRegister},
    rams::{HighRam, ObjectAttributeMem, VedioRam, WorkRam},
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
    hram: HighRam,
    intr_enable_reg: InterruptEnableRegsiter,
    intr_mask_reg: InterruptMaskRegister,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            cartridge: None,
            vram: VedioRam::new(),
            wram: WorkRam::new(),
            oam: ObjectAttributeMem::new(),
            hram: HighRam::new(),
            intr_mask_reg: InterruptMaskRegister::new(),
            intr_enable_reg: InterruptEnableRegsiter::new(),
        }
    }

    pub fn read(&self, addr: Addr) -> Result<Word> {
        match addr {
            CART_ROM_LOW_BOUND..=CART_ROM_HIGH_BOUND_INCLUDED
            | CART_RAM_LOW_BOUND..=CART_RAM_HIGH_BOUND_INCLUDED => {
                if let Some(ref c) = self.cartridge {
                    c.read(addr)
                } else {
                    warn!("no cartridge is plugged in! illegal read at address: 0x:{addr:04X}");
                    Err(EmulatorError::NoCartridge)
                }
            }
            VRAM_LOW_BOUND..=VRAM_HIGH_BOUND_INCLUDED => self.vram.read(addr),
            WRAM_LOW_BOUND..=WRAM_HIGH_BOUND_INCLUDED => self.wram.read(addr),
            OAM_LOW_BOUND..=OAM_HIGH_BOUND_INCLUDED => self.oam.read(addr),
            HRAM_LOW_BOUND..=HRAM_HIGH_BOUND_INCLUDED => self.hram.read(addr),
            INTERRUPT_ENABLE_REGISTER_ADDR => self.intr_enable_reg.read(),
            INTERRUPT_MASKS_REGISTER_ADDR => self.intr_mask_reg.read(),
            _ => {
                warn!("illegal read at address: 0x{addr:04X}");
                Ok(0xFF)
            }
        }
    }

    pub fn write(&mut self, addr: Addr, data: Word) -> Result {
        match addr {
            CART_ROM_LOW_BOUND..=CART_ROM_HIGH_BOUND_INCLUDED
            | CART_RAM_LOW_BOUND..=CART_RAM_HIGH_BOUND_INCLUDED => {
                if let Some(ref mut c) = self.cartridge {
                    c.write(addr, data)
                } else {
                    warn!("no cartridge is plugged in! illegal write at address: 0x:{addr:04X}");
                    Err(EmulatorError::NoCartridge)
                }
            }
            VRAM_LOW_BOUND..=VRAM_HIGH_BOUND_INCLUDED => self.vram.write(addr, data),
            WRAM_LOW_BOUND..=WRAM_HIGH_BOUND_INCLUDED => self.wram.write(addr, data),
            OAM_LOW_BOUND..=OAM_HIGH_BOUND_INCLUDED => self.oam.write(addr, data),
            HRAM_LOW_BOUND..=HRAM_HIGH_BOUND_INCLUDED => self.hram.write(addr, data),
            INTERRUPT_ENABLE_REGISTER_ADDR => self.intr_enable_reg.write(data),
            INTERRUPT_MASKS_REGISTER_ADDR => self.intr_mask_reg.write(data),
            _ => {
                warn!("illegal write at address: 0x{addr:04X}");
                Ok(())
            }
        }
    }

    pub fn reset(&mut self) {
        todo!()
    }

    pub fn intr_enable_reg(&self) -> InterruptEnableRegsiter {
        self.intr_enable_reg
    }

    pub fn intr_enable_reg_mut(&mut self) -> &mut InterruptEnableRegsiter {
        &mut self.intr_enable_reg
    }

    pub fn intr_mask_reg(&self) -> InterruptMaskRegister {
        self.intr_mask_reg
    }

    pub fn intr_mask_reg_mut(&mut self) -> &mut InterruptMaskRegister {
        &mut self.intr_mask_reg
    }
}

pub const CART_ROM_LOW_BOUND: Addr = 0x0000;
pub const VRAM_LOW_BOUND: Addr = 0x8000;
pub const CART_RAM_LOW_BOUND: Addr = 0xA000;
pub const WRAM_LOW_BOUND: Addr = 0xC000;
pub const OAM_LOW_BOUND: Addr = 0xFE00;
pub const IO_LOW_BOUND: Addr = 0xFF00;
pub const HRAM_LOW_BOUND: Addr = 0xFF80;
pub const INTERRUPT_ENABLE_REGISTER_ADDR: Addr = 0xFFFF;

pub const CART_ROM_HIGH_BOUND: Addr = VRAM_LOW_BOUND;
pub const VRAM_HIGH_BOUND: Addr = CART_RAM_LOW_BOUND;
pub const CART_RAM_HIGH_BOUND: Addr = WRAM_LOW_BOUND;
pub const WRAM_HIGH_BOUND: Addr = 0xE000;
pub const OAM_HIGH_BOUND: Addr = 0xFEA0;
pub const IO_HIGH_BOUND: Addr = HRAM_LOW_BOUND;
pub const HRAM_HIGH_BOUND: Addr = INTERRUPT_ENABLE_REGISTER_ADDR;

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

pub const INTERRUPT_MASKS_REGISTER_ADDR: Addr = 0xFF0F;

pub trait BusDevice {
    /// 默认返回0xFF
    fn read(&self, addr: Addr) -> Result<Word> {
        warn!("illegal read at address: 0x{addr:04X}");
        Ok(0xFF)
    }

    #[allow(unused)]
    fn write(&mut self, addr: Addr, data: Word) -> Result {
        warn!("illegal write at address: 0x{addr:04X}");
        Ok(())
    }
}
