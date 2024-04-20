use crate::dev::bus::Bus;

use self::regs::Regs;

pub mod regs;

pub struct CPU {
    regs: Regs,
    halted: bool,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            regs: Default::default(),
            halted: true,
        }
    }

    pub fn step(&mut self, bus: &mut Bus) {
        todo!()
    }

    pub fn reset(&mut self) {
        todo!()
    }

    pub fn boot(&mut self) {
        todo!()
    }
}
