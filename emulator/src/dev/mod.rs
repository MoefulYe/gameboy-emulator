use crate::types::{Addr, Word};
use int_regs::IRQ;
use log::warn;
use std::default::Default;

pub mod apu;
pub mod bus;
pub mod cart;
pub mod cpu;
pub mod gamepad;
pub mod int_regs;
pub mod ppu;
pub mod rams;
pub mod serial;
pub mod timer;

pub trait Reset {
    fn reset(&mut self);
}

impl<T> Reset for T
where
    T: Default,
{
    fn reset(&mut self) {
        *self = Default::default();
    }
}

pub trait MemoryRegion {
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

pub use bus::Bus;
pub use cart::{Cart, LoadCartResult};
pub use cpu::CPU;
