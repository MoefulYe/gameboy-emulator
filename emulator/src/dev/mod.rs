use crate::types::{Addr, Word};
use log::warn;
use std::default::Default;

mod bus;
mod cartridge;
mod cpu;
mod gamepad;
mod int_regs;
mod rams;
mod serial;
mod timer;

pub trait Resetable {
    fn reset(&mut self);
}

impl<T> Resetable for T
where
    T: Default,
{
    fn reset(&mut self) {
        *self = Default::default();
    }
}

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

#[derive(PartialEq, Eq)]
pub enum TickResult {
    IntReq,
    Ok,
}

impl TickResult {
    pub fn int_req(self) -> bool {
        self == TickResult::IntReq
    }
}

pub trait Tickable: BusDevice {
    fn tick(&mut self) -> TickResult;
}

pub use bus::Bus;
pub use cartridge::PluginCartResult;
pub use cpu::CPU;
