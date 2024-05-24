use super::bus::NO_BREAK;
use crate::{
    dev::{
        bus::Bus,
        cpu::{ime::InterruptMasterEnableRegsiter, inst::Inst, regs::Regs},
    },
    error::Result,
    trace::CPUState,
    types::{Addr, ClockCycle, OpCode, Word},
};
use std::ops::{Deref, DerefMut};

mod cb;
mod ime;
mod inst;
mod regs;

pub struct CPU {
    regs: Regs,
    halted: bool,
    ime: InterruptMasterEnableRegsiter,
}

impl DerefMut for CPU {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.regs
    }
}

impl Deref for CPU {
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

    /// 返回花费的时钟周期和是否触发断点
    pub fn tick(&mut self, bus: &mut Bus) -> Result<(ClockCycle, bool)> {
        if !self.halted {
            if let Some(int_entry) = bus.int_entry() {
                self.handle_int(bus, int_entry)
            } else {
                let (opcode, brk0) = self.fetch_opcode(bus)?;
                self.pc_inc();
                let inst = Self::decode_inst(opcode);
                let (cycles, brk1) = self.exec_inst(bus, inst)?;
                self.ime.countdown();
                Ok((cycles, brk0 || brk1))
            }
        } else {
            if bus.has_int() {
                self.halted = false;
            }
            self.ime.countdown();
            Ok((4, NO_BREAK))
        }
    }

    pub fn trace(&self, bus: &mut Bus, pc: Addr) -> Box<CPUState> {
        let inst = bus
            .read(pc)
            .map(|(op, _)| Self::mnemonic(op))
            .unwrap_or("UNKNOWN");
        let pc = self.pc();
        let three_words_at_pc = [
            bus.read(pc).map(|(w, _)| w).unwrap_or(0),
            bus.read(pc + 1).map(|(w, _)| w).unwrap_or(0),
            bus.read(pc + 2).map(|(w, _)| w).unwrap_or(0),
        ];
        Box::new(CPUState {
            ime: self.ime.enabled(),
            halted: self.halted,
            a: self.a(),
            f: self.f(),
            b: self.b(),
            c: self.c(),
            d: self.d(),
            e: self.e(),
            h: self.h(),
            l: self.l(),
            af: self.af(),
            bc: self.bc(),
            de: self.de(),
            hl: self.hl(),
            pc: self.pc(),
            sp: self.sp(),
            zero_flag: self.zero_flag(),
            negative_flag: self.negative_flag(),
            half_flag: self.half_carry_flag(),
            carry_flag: self.carry_flag(),
            inst,
            three_words_at_pc,
        })
    }

    pub fn reset(&mut self) {
        todo!()
    }

    fn handle_int(&mut self, bus: &mut Bus, entry: Addr) -> Result<(ClockCycle, bool)> {
        self.ime.disable();
        let brk = self.push_dword(bus, self.pc())?;
        self.jp(entry);
        Ok((20, brk))
    }

    fn fetch_opcode(&self, bus: &Bus) -> Result<(OpCode, bool)> {
        bus.read(self.pc())
    }

    fn exec_inst(&mut self, bus: &mut Bus, inst: Inst) -> Result<(ClockCycle, bool)> {
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
