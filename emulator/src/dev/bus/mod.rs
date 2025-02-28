use super::{
    cart::{Cart, CartInfo},
    gamepad::{Buttons, BUTTON_ADDR},
    int_regs::{
        InterruptFlagRegister, InterruptMaskRegsiter, INTERRUPT_FLAG_REGISTER_ADDR,
        INTERRUPT_MASK_REGISTER_ADDR, INT_JOYPAD_ENTRY, INT_LCD_STAT_ENTRY, INT_LCD_STAT_MASK,
        INT_SERIAL_ENTRY, INT_SERIAL_MASK, INT_TIMER_ENTRY, INT_TIMER_MASK, INT_VBLANK_ENTRY,
        INT_VBLANK_MASK,
    },
    ppu::{PPU, PPU_ADDR_HIGH_BOUND_INCLUDED, PPU_ADDR_LOW_BOUND},
    rams::{HighRam, WRAM},
    serial::{Serial, SERIAL_ADDR_HIGH_BOUND_INCLUDED, SERIAL_ADDR_LOW_BOUND},
    timer::{Timer, TIMER_ADDR_HIGH_BOUND_INCLUDED, TIMER_ADDR_LOW_BOUND},
    BusDevice, Reset, 
};
use crate::{
    error::{EmuErr, EmuResult, NoCartridge},
    types::{Addr, Word},
};
use log::warn;
use serde::{Deserialize, Serialize};

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
#[derive(Serialize, Deserialize)]
pub struct Bus {
    pub cart: Option<Cart>,
    pub wram: WRAM,
    pub serial: Serial,
    pub ppu: PPU,
    pub timer: Timer,
    pub btns: Buttons,
    pub hram: HighRam,
    pub int_flag_reg: InterruptFlagRegister,
    pub int_mask_reg: InterruptMaskRegsiter,
}

impl Reset for Bus {
    fn reset(&mut self) {
        self.cart = None;
        self.wram.reset();
        self.serial.reset();
        self.ppu.reset();
        self.timer.reset();
        self.btns.reset();
        self.hram.reset();
        self.int_flag_reg.reset();
        self.int_mask_reg.reset();
    }
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            cart: None,
            wram: WRAM::new(),
            serial: Serial::new(),
            ppu: PPU::new(),
            timer: Timer::new(),
            int_flag_reg: InterruptFlagRegister::new(),
            hram: HighRam::new(),
            int_mask_reg: InterruptMaskRegsiter::new(),
            btns: Default::default(),
        }
    }

    pub fn read(&mut self, addr: Addr) -> EmuResult<Word> {
        let word = match addr {
            CART_ROM_LOW_BOUND..=CART_ROM_HIGH_BOUND_INCLUDED
            | CART_RAM_LOW_BOUND..=CART_RAM_HIGH_BOUND_INCLUDED => {
                if let Some(ref c) = self.cart {
                    c.read(addr)
                } else {
                    warn!("no cartridge is plugged in! illegal read at address: 0x:{addr:04X}");
                    return EmuErr(NoCartridge);
                }
            }
            VRAM_LOW_BOUND..=VRAM_HIGH_BOUND_INCLUDED => self.ppu.vram.read(addr),
            WRAM_LOW_BOUND..=WRAM_HIGH_BOUND_INCLUDED => self.wram.read(addr),
            OAM_LOW_BOUND..=OAM_HIGH_BOUND_INCLUDED => self.ppu.oam.read(addr),
            BUTTON_ADDR => self.btns.read(),
            SERIAL_ADDR_LOW_BOUND..=SERIAL_ADDR_HIGH_BOUND_INCLUDED => self.serial.read(addr),
            TIMER_ADDR_LOW_BOUND..=TIMER_ADDR_HIGH_BOUND_INCLUDED => self.timer.read(addr),
            PPU_ADDR_LOW_BOUND..=PPU_ADDR_HIGH_BOUND_INCLUDED => self.ppu.read(addr),
            INTERRUPT_FLAG_REGISTER_ADDR => self.int_flag_reg.read(),
            HRAM_LOW_BOUND..=HRAM_HIGH_BOUND_INCLUDED => self.hram.read(addr),
            INTERRUPT_MASK_REGISTER_ADDR => self.int_mask_reg.read(),
            _ => {
                warn!("illegal read at address: 0x{addr:04X}");
                0xFF
            }
        };
        Ok(word)
    }

    pub fn write(&mut self, addr: Addr, data: Word) -> EmuResult<()> {
        match addr {
            CART_ROM_LOW_BOUND..=CART_ROM_HIGH_BOUND_INCLUDED
            | CART_RAM_LOW_BOUND..=CART_RAM_HIGH_BOUND_INCLUDED => {
                if let Some(ref mut c) = self.cart {
                    c.write(addr, data)
                } else {
                    warn!("no cartridge is plugged in! illegal write at address: 0x:{addr:04X}");
                    return EmuErr(NoCartridge);
                }
            }
            VRAM_LOW_BOUND..=VRAM_HIGH_BOUND_INCLUDED => self.ppu.vram.write(addr, data),
            WRAM_LOW_BOUND..=WRAM_HIGH_BOUND_INCLUDED => self.wram.write(addr, data),
            OAM_LOW_BOUND..=OAM_HIGH_BOUND_INCLUDED => self.ppu.oam.write(addr, data),
            BUTTON_ADDR => self.btns.write(data),
            SERIAL_ADDR_LOW_BOUND..=SERIAL_ADDR_HIGH_BOUND_INCLUDED => {
                self.serial.write(addr, data)
            }
            TIMER_ADDR_LOW_BOUND..=TIMER_ADDR_HIGH_BOUND_INCLUDED => self.timer.write(addr, data),
            PPU_ADDR_LOW_BOUND..=PPU_ADDR_HIGH_BOUND_INCLUDED => self.ppu.write(addr, data),
            INTERRUPT_FLAG_REGISTER_ADDR => self.int_flag_reg.write(data),
            HRAM_LOW_BOUND..=HRAM_HIGH_BOUND_INCLUDED => self.hram.write(addr, data),
            INTERRUPT_MASK_REGISTER_ADDR => self.int_mask_reg.write(data),
            _ => warn!("illegal write at address: 0x{addr:04X}"),
        };
        Ok(())
    }

    pub fn tick_dma(&mut self) -> EmuResult {
        if let Some((hi, lo)) = self.ppu.dma.tick() {
            let addr = (hi as Addr) << 8 | (lo as Addr);
            let data = self.read(addr)?;
            unsafe { *self.ppu.oam.get_unchecked_mut(lo as usize) = data }
        }
        Ok(())
    }

    /// 是否有中断事件等待处理
    pub fn has_int(&self) -> bool {
        self.int_flag_reg.val() & self.int_mask_reg.val() != 0
    }

    pub fn int_entry(&mut self, ime: bool) -> Option<Addr> {
        if !ime {
            return None;
        }
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

    pub fn load_cart(&mut self, rom: Box<[u8]>, timestamp: i64) -> EmuResult<CartInfo> {
        let cart = Cart::new(rom, timestamp)?;
        let info = cart.header().info();
        self.cart = Some(cart);
        Ok(info)
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
