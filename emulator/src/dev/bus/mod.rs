use self::breakpoint::BreakPoints;
use super::{
    cartridge::{Cartridge, PluginCartResult},
    int_regs::{
        InterruptFlagRegister, InterruptMaskRegsiter, INTERRUPT_FLAG_REGISTER_ADDR,
        INTERRUPT_MASK_REGISTER_ADDR, INT_JOYPAD_ENTRY, INT_LCD_STAT_ENTRY, INT_LCD_STAT_MASK,
        INT_SERIAL_ENTRY, INT_SERIAL_MASK, INT_TIMER_ENTRY, INT_TIMER_MASK, INT_VBLANK_ENTRY,
        INT_VBLANK_MASK,
    },
    rams::{HighRam, ObjectAttributeMem, VedioRam, WorkRam},
    serial::{Serial, SERIAL_TRANSFER_ADDR_HIGH_BOUND_INCLUDED, SERIAL_TRANSFER_ADDR_LOW_BOUND},
    timer::{Timer, TIMER_ADDR_HIGH_BOUND_INCLUDED, TIMER_ADDR_LOW_BOUND},
    BusDevice, Tickable,
};
use crate::{
    error::{BoxedEmulatorError, BoxedEmulatorErrorInfo, EmulatorError, Result},
    types::{Addr, Word},
};
use log::warn;

mod breakpoint;

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
    serial: Serial,
    timer: Timer,
    int_flag_reg: InterruptFlagRegister,
    hram: HighRam,
    int_mask_reg: InterruptMaskRegsiter,
    breakpoints: BreakPoints,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            cartridge: None,
            vram: VedioRam::new(),
            wram: WorkRam::new(),
            oam: ObjectAttributeMem::new(),
            serial: Serial::new(),
            timer: Timer::new(),
            int_flag_reg: InterruptFlagRegister::new(),
            hram: HighRam::new(),
            int_mask_reg: InterruptMaskRegsiter::new(),
            breakpoints: BreakPoints::new(),
        }
    }

    pub fn read(&self, addr: Addr) -> Result<(Word, bool)> {
        let brk = self.breakpoints.break_memread(addr);
        let word = match addr {
            CART_ROM_LOW_BOUND..=CART_ROM_HIGH_BOUND_INCLUDED
            | CART_RAM_LOW_BOUND..=CART_RAM_HIGH_BOUND_INCLUDED => {
                if let Some(ref c) = self.cartridge {
                    c.read(addr)
                } else {
                    warn!("no cartridge is plugged in! illegal read at address: 0x:{addr:04X}");
                    return Err(Box::new(EmulatorError::NoCartridge));
                }
            }
            VRAM_LOW_BOUND..=VRAM_HIGH_BOUND_INCLUDED => self.vram.read(addr),
            WRAM_LOW_BOUND..=WRAM_HIGH_BOUND_INCLUDED => self.wram.read(addr),
            OAM_LOW_BOUND..=OAM_HIGH_BOUND_INCLUDED => self.oam.read(addr),
            SERIAL_TRANSFER_ADDR_LOW_BOUND..=SERIAL_TRANSFER_ADDR_HIGH_BOUND_INCLUDED => {
                self.serial.read(addr)
            }
            TIMER_ADDR_LOW_BOUND..=TIMER_ADDR_HIGH_BOUND_INCLUDED => self.timer.read(addr),
            INTERRUPT_FLAG_REGISTER_ADDR => self.int_flag_reg.read(),
            HRAM_LOW_BOUND..=HRAM_HIGH_BOUND_INCLUDED => self.hram.read(addr),
            INTERRUPT_MASK_REGISTER_ADDR => self.int_mask_reg.read(),
            _ => {
                warn!("illegal read at address: 0x{addr:04X}");
                0xFF
            }
        };
        Ok((word, brk))
    }

    pub fn write(&mut self, addr: Addr, data: Word) -> Result<bool> {
        let brk = self.breakpoints.break_memwrite(addr);
        match addr {
            CART_ROM_LOW_BOUND..=CART_ROM_HIGH_BOUND_INCLUDED
            | CART_RAM_LOW_BOUND..=CART_RAM_HIGH_BOUND_INCLUDED => {
                if let Some(ref mut c) = self.cartridge {
                    c.write(addr, data)
                } else {
                    warn!("no cartridge is plugged in! illegal write at address: 0x:{addr:04X}");
                    return Err(Box::new(EmulatorError::NoCartridge));
                }
            }
            VRAM_LOW_BOUND..=VRAM_HIGH_BOUND_INCLUDED => self.vram.write(addr, data),
            WRAM_LOW_BOUND..=WRAM_HIGH_BOUND_INCLUDED => self.wram.write(addr, data),
            OAM_LOW_BOUND..=OAM_HIGH_BOUND_INCLUDED => self.oam.write(addr, data),
            SERIAL_TRANSFER_ADDR_LOW_BOUND..=SERIAL_TRANSFER_ADDR_HIGH_BOUND_INCLUDED => {
                self.serial.write(addr, data)
            }
            TIMER_ADDR_LOW_BOUND..=TIMER_ADDR_HIGH_BOUND_INCLUDED => self.timer.write(addr, data),
            INTERRUPT_FLAG_REGISTER_ADDR => self.int_flag_reg.write(data),
            HRAM_LOW_BOUND..=HRAM_HIGH_BOUND_INCLUDED => self.hram.write(addr, data),
            INTERRUPT_MASK_REGISTER_ADDR => self.int_mask_reg.write(data),
            _ => warn!("illegal write at address: 0x{addr:04X}"),
        };
        Ok(brk)
    }

    pub fn reset(&mut self) {
        todo!()
    }

    pub fn tick(&mut self) -> bool {
        let mut brk = NO_BREAK;
        let timer_int = self.timer.tick().int_req();
        if timer_int {
            self.int_flag_reg.set_timer_int();
            brk |= self.breakpoints.break_timer();
        }
        let serial_int = self.serial.tick().int_req();
        if serial_int {
            self.int_flag_reg.set_serial_int();
            brk |= self.breakpoints.break_serial();
        }
        brk
    }
    /// 是否有中断事件等待处理
    pub fn has_int(&self) -> bool {
        self.int_flag_reg.val() & self.int_mask_reg.val() != 0
    }

    pub fn int_entry(&mut self) -> Option<Addr> {
        let flags = self.int_flag_reg.val() & self.int_mask_reg.val();
        if flags == 0 {
            None
        } else if flags & INT_VBLANK_MASK != 0 {
            self.int_flag_reg.clear_vblank_int();
            Some(INT_VBLANK_ENTRY)
        } else if flags & INT_LCD_STAT_MASK != 0 {
            self.int_flag_reg.clear_lcd_stat_int();
            Some(INT_LCD_STAT_ENTRY)
        } else if flags & INT_TIMER_MASK != 0 {
            self.int_flag_reg.clear_timer_int();
            Some(INT_TIMER_ENTRY)
        } else if flags & INT_SERIAL_MASK != 0 {
            self.int_flag_reg.clear_serial_int();
            Some(INT_SERIAL_ENTRY)
        } else {
            self.int_flag_reg.clear_joypad_int();
            Some(INT_JOYPAD_ENTRY)
        }
    }

    pub fn serial(&self) -> &Serial {
        &self.serial
    }

    pub fn serial_mut(&mut self) -> &mut Serial {
        &mut self.serial
    }

    pub fn plugin_cart(&mut self, cartridge: Box<[u8]>) -> PluginCartResult {
        let cartridge = Cartridge::new(cartridge);
        let res = cartridge.check_and_get_info();
        self.cartridge = Some(cartridge);
        res
    }
    pub fn plugout_cart(&mut self) {
        self.cartridge = None
    }
}

pub const CART_ROM_LOW_BOUND: Addr = 0x0000;
pub const VRAM_LOW_BOUND: Addr = 0x8000;
pub const CART_RAM_LOW_BOUND: Addr = 0xA000;
pub const WRAM_LOW_BOUND: Addr = 0xC000;
pub const OAM_LOW_BOUND: Addr = 0xFE00;
pub const IO_LOW_BOUND: Addr = 0xFF00;
pub const HRAM_LOW_BOUND: Addr = 0xFF80;

pub const CART_ROM_HIGH_BOUND: Addr = VRAM_LOW_BOUND;
pub const VRAM_HIGH_BOUND: Addr = CART_RAM_LOW_BOUND;
pub const CART_RAM_HIGH_BOUND: Addr = WRAM_LOW_BOUND;
pub const WRAM_HIGH_BOUND: Addr = 0xE000;
pub const OAM_HIGH_BOUND: Addr = 0xFEA0;
pub const IO_HIGH_BOUND: Addr = HRAM_LOW_BOUND;
pub const HRAM_HIGH_BOUND: Addr = INTERRUPT_MASK_REGISTER_ADDR;

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

pub use breakpoint::{Break, BREAK, NO_BREAK};
