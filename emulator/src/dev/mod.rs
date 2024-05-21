use crate::{
    error::Result,
    types::{Addr, Word},
};
use log::warn;
use std::default::Default;

pub mod bus;
pub mod buttons;
pub mod clock;
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
