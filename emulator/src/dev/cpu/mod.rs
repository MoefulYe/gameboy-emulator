use crate::{
    dev::{
        bus::Bus,
        cpu::{ime::InterruptMasterEnableRegsiter, inst::Inst, regs::Regs},
    },
    dump::CPUStateDump,
    error::EmuResult,
    types::{Addr, ClockCycle, OpCode, Word},
};
use std::ops::{Deref, DerefMut};

mod cb;
mod ime;
mod inst;
mod regs;

#[derive(Default)]
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
        Default::default()
    }

    /// 返回花费的时钟周期
    pub fn tick(&mut self, bus: &mut Bus) -> EmuResult<ClockCycle> {
        if !self.halted {
            let ime = self.ime.enabled();
            if let Some(int_entry) = bus.int_entry(ime) {
                self.handle_int(bus, int_entry)
            } else {
                let opcode = self.fetch_opcode(bus)?;
                self.pc_inc();
                let inst = Self::decode_inst(opcode);
                let cycles = self.exec_inst(bus, inst)?;
                self.ime.countdown();
                Ok(cycles)
            }
        } else {
            if bus.has_int() {
                self.halted = false;
            }
            self.ime.countdown();
            Ok(4)
        }
    }

    pub fn dump(&self, bus: &Bus) -> CPUStateDump {
        let pc = self.pc();
        let inst = bus
            .read(pc)
            .map(|op| Self::mnemonic(op))
            .unwrap_or("UNKNOWN");
        let three_words_at_pc = [
            bus.read(pc).unwrap_or(0),
            bus.read(pc + 1).unwrap_or(0),
            bus.read(pc + 2).unwrap_or(0),
        ];
        CPUStateDump {
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
        }
    }

    pub fn reset(&mut self) {
        *self = Default::default()
    }

    fn handle_int(&mut self, bus: &mut Bus, entry: Addr) -> EmuResult<ClockCycle> {
        self.ime.disable();
        self.push_dword(bus, self.pc())?;
        self.jp(entry);
        Ok(20)
    }

    fn fetch_opcode(&self, bus: &Bus) -> EmuResult<OpCode> {
        bus.read(self.pc())
    }

    fn exec_inst(&mut self, bus: &mut Bus, inst: Inst) -> EmuResult<ClockCycle> {
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
