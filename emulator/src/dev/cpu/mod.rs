use crate::{
    dev::{
        bus::Bus,
        cpu::{ime::InterruptMasterEnableRegsiter, inst::Inst, regs::Regs},
    },
    error::Result,
    types::{Addr, ClockCycle, OpCode, Word},
};

mod cb;
mod ime;
mod inst;
mod regs;

pub struct CPU {
    regs: Regs,
    halted: bool,
    ime: InterruptMasterEnableRegsiter,
}

impl std::ops::DerefMut for CPU {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.regs
    }
}

impl std::ops::Deref for CPU {
    type Target = Regs;

    fn deref(&self) -> &Self::Target {
        &self.regs
    }
}

impl CPU {
    pub fn new() -> Self {
        Self {
            regs: Regs::new(),
            halted: false,
            ime: InterruptMasterEnableRegsiter::new(),
        }
    }

    /// 返回花费的时钟周期
    pub fn tick(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        if !self.halted {
            if let Some(int_entry) = bus.int_entry() {
                self.handle_int(bus, int_entry)
            } else {
                let opcode = self.fetch_opcode(bus)?;
                self.pc_inc();
                let inst = Self::decode_inst(opcode);
                let res = self.exec_inst(bus, inst)?;
                self.ime.countdown();
                Ok(res)
            }
        } else {
            if bus.has_int() {
                self.halted = false;
            }
            self.ime.countdown();
            Ok(4)
        }
    }

    pub fn reset(&mut self) {
        todo!()
    }

    fn handle_int(&mut self, bus: &mut Bus, entry: Addr) -> Result<ClockCycle> {
        self.ime.disable();
        self.push_dword(bus, self.pc())?;
        self.jp(entry);
        Ok(20)
    }

    fn fetch_opcode(&self, bus: &Bus) -> Result<OpCode> {
        bus.read(self.pc())
    }

    fn exec_inst(&mut self, bus: &mut Bus, inst: Inst) -> Result<ClockCycle> {
        inst(self, bus)
    }

    fn pc_inc(&mut self) {
        *self.pc_mut() += 1;
    }

    fn pc_inc_by(&mut self, n: Addr) {
        *self.pc_mut() += n;
    }

    fn jp(&mut self, addr: Addr) {
        *self.pc_mut() = addr;
    }

    fn jr(&mut self, offset: Word) {
        let pc = self.pc_mut();
        let offset = offset as i8 as i16 as Addr;
        *pc = pc.wrapping_add(offset)
    }
}
