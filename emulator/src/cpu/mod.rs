use crate::{
    dev::bus::Bus,
    types::{OpCode, Word},
};

use self::regs::Regs;

type Inst = fn(&mut CPU, &mut Bus) -> u32;

pub mod regs;

pub struct CPU {
    regs: Regs,
    halted: bool,
}

impl CPU {
    pub fn new() -> Self {
        todo!("初始状态要参考文档")
    }

    /// 返回花费的时钟周期
    pub fn clock(&mut self, bus: &mut Bus) -> u32 {
        todo!()
    }

    pub fn fetch_inst(&self, bus: &Bus) -> OpCode {
        bus.read(self.pc())
    }

    #[inline]
    pub fn decode_inst(opcode: OpCode) -> Inst {
        todo!()
    }

    pub fn reset(&mut self) {
        todo!()
    }

    pub fn boot(&mut self) {
        todo!()
    }

    pub fn regs(&self) -> &Regs {
        &self.regs
    }

    pub fn regs_mut(&mut self) -> &mut Regs {
        &mut self.regs
    }

    pub fn halted(&self) -> bool {
        self.halted
    }

    pub fn pc(&self) -> crate::types::Addr {
        self.regs.pc()
    }
}
