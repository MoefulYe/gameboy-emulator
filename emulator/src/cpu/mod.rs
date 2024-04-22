use log::error;

use crate::{
    dev::bus::{Bus, IO_LOW_BOUND},
    types::{Addr, ClockCycle, OpCode, Word},
};

use self::regs::Regs;

type Inst = fn(&mut CPU, &mut Bus) -> ClockCycle;

pub mod regs;

pub struct CPU {
    regs: Regs,
    halted: bool,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            regs: Regs::new(),
            halted: false,
        }
    }

    /// 返回花费的时钟周期
    pub fn clock(&mut self, bus: &mut Bus) -> ClockCycle {
        let opcode = self.fetch_opcode(bus);
        self.pc_inc();
        let inst = Self::decode_inst(opcode);
        self.exec_inst(bus, inst)
    }

    fn fetch_opcode(&self, bus: &Bus) -> OpCode {
        bus.read(self.pc())
    }

    fn decode_inst(opcode: OpCode) -> Inst {
        const INSTS: &[Inst; 256] = &[];
        INSTS[opcode as usize]
    }

    fn mnemonic(opcode: OpCode) -> &'static str {
        const MNEMONICS: &[&'static str; 256] = &[];
        MNEMONICS[opcode as usize]
    }

    fn exec_inst(&mut self, bus: &mut Bus, inst: Inst) -> ClockCycle {
        inst(self, bus)
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

    pub fn pc(&self) -> Addr {
        self.regs.pc()
    }

    fn pc_inc(&mut self) {
        *self.regs.pc_mut() += 1;
    }

    fn pc_dec(&mut self) {
        *self.regs.pc_mut() -= 1;
    }

    fn pc_inc_by(&mut self, n: Addr) {
        *self.regs.pc_mut() += n;
    }

    fn pc_dec_by(&mut self, n: Addr) {
        *self.regs.pc_mut() -= n;
    }

    fn jp(&mut self, addr: Addr) {
        *self.regs.pc_mut() = addr;
    }
}

/// ref https://gbdev.io/pandocs/CPU_Instruction_Set.html
/// ref https://gbdev.io/gb-opcodes/optables/
/// ref https://archive.org/details/GameBoyProgManVer1.1/page/n93/mode/2up
/// Misc / control instructions
impl CPU {
    fn inst_0x00_nop(_: &mut CPU, _: &mut Bus) -> ClockCycle {
        4
    }

    fn illegal_opcode(&mut self, bus: &mut Bus) -> ClockCycle {
        let addr = self.pc() - 1;
        let opcode = bus.read(addr);
        error!("illegal opcode: 0x{opcode:02X} at address: 0x{addr:04X}",);
        todo!()
    }
}

/// LD between 8bit registers instructions
/// LD dest, src
impl CPU {
    fn inst_0x40_ld_b_b(_: &mut CPU, _: &mut Bus) -> ClockCycle {
        4
    }

    fn inst_0x41_ld_b_c(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.b_mut() = regs.c();
        4
    }

    fn inst_0x42_ld_b_d(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.b_mut() = regs.d();
        4
    }

    fn inst_0x43_ld_b_e(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.b_mut() = regs.e();
        4
    }

    fn inst_0x44_ld_b_h(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.b_mut() = regs.h();
        4
    }

    fn inst_0x45_ld_b_l(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.b_mut() = regs.l();
        4
    }

    fn inst_0x47_ld_b_a(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.b_mut() = regs.a();
        4
    }

    fn inst_0x48_ld_c_b(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.c_mut() = regs.b();
        4
    }

    fn inst_0x49_ld_c_c(_: &mut CPU, _: &mut Bus) -> ClockCycle {
        4
    }

    fn inst_0x4a_ld_c_d(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.c_mut() = regs.d();
        4
    }

    fn inst_0x4b_ld_c_e(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.c_mut() = regs.e();
        4
    }

    fn inst_0x4c_ld_c_h(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.c_mut() = regs.h();
        4
    }

    fn inst_0x4d_ld_c_l(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.c_mut() = regs.l();
        4
    }

    fn inst_0x4f_ld_c_a(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.c_mut() = regs.a();
        4
    }

    fn inst_0x50_ld_d_b(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.d_mut() = regs.b();
        4
    }

    fn inst_0x51_ld_d_c(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.d_mut() = regs.c();
        4
    }

    fn inst_0x52_ld_d_d(_: &mut CPU, _: &mut Bus) -> ClockCycle {
        4
    }

    fn inst_0x53_ld_d_e(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.d_mut() = regs.e();
        4
    }

    fn inst_0x54_ld_d_h(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.d_mut() = regs.h();
        4
    }

    fn inst_0x55_ld_d_l(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.d_mut() = regs.l();
        4
    }

    fn inst_0x57_ld_d_a(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.d_mut() = regs.a();
        4
    }

    fn inst_0x58_ld_e_b(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.e_mut() = regs.b();
        4
    }

    fn inst_0x59_ld_e_c(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.e_mut() = regs.c();
        4
    }

    fn inst_0x5a_ld_e_d(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.e_mut() = regs.d();
        4
    }

    fn inst_0x5b_ld_e_e(&mut self, _: &mut Bus) -> ClockCycle {
        4
    }

    fn inst_0x5c_ld_e_h(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.e_mut() = regs.h();
        4
    }

    fn inst_0x5d_ld_e_l(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.e_mut() = regs.l();
        4
    }

    fn inst_0x5f_ld_e_a(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.e_mut() = regs.a();
        4
    }

    fn inst_0x60_ld_h_b(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.h_mut() = regs.b();
        4
    }

    fn inst_0x61_ld_h_c(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.h_mut() = regs.c();
        4
    }

    fn inst_0x62_ld_h_d(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.h_mut() = regs.d();
        4
    }

    fn inst_0x63_ld_h_e(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.h_mut() = regs.e();
        4
    }

    fn inst_0x64_ld_h_h(&mut self, _: &mut Bus) -> ClockCycle {
        4
    }

    fn inst_0x65_ld_h_l(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.h_mut() = regs.l();
        4
    }

    fn inst_0x67_ld_h_a(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.h_mut() = regs.a();
        4
    }

    fn inst_0x68_ld_l_b(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.l_mut() = regs.b();
        4
    }

    fn inst_0x69_ld_l_c(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.l_mut() = regs.c();
        4
    }

    fn inst_0x6a_ld_l_d(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.l_mut() = regs.d();
        4
    }

    fn inst_0x6b_ld_l_e(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.l_mut() = regs.e();
        4
    }

    fn inst_0x6c_ld_l_h(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.l_mut() = regs.h();
        4
    }

    fn inst_0x6d_ld_l_l(&mut self, _: &mut Bus) -> ClockCycle {
        4
    }

    fn inst_0x6f_ld_l_a(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.l_mut() = regs.a();
        4
    }

    fn inst_0x78_ld_a_b(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.a_mut() = regs.b();
        4
    }

    fn inst_0x79_ld_a_c(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.a_mut() = regs.c();
        4
    }

    fn inst_0x7a_ld_a_d(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.a_mut() = regs.d();
        4
    }

    fn inst_0x7b_ld_a_e(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.a_mut() = regs.e();
        4
    }

    fn inst_0x7c_ld_a_h(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.a_mut() = regs.h();
        4
    }

    fn inst_0x7d_ld_a_l(&mut self, _: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        *regs.a_mut() = regs.l();
        4
    }

    fn inst_0x7f_ld_a_a(&mut self, _: &mut Bus) -> ClockCycle {
        4
    }
}

/// LD from memory to 8bit register instructions
/// LD dest, (16 bits register pointers to memory)
impl CPU {
    fn inst_0x0a_ld_a_bc(&mut self, bus: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        let bc = regs.bc();
        let data = bus.read(bc);
        *regs.a_mut() = data;
        8
    }

    fn inst_0x1a_ld_a_de(&mut self, bus: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        let de = regs.de();
        let data = bus.read(de);
        *regs.a_mut() = data;
        8
    }

    fn inst_0x46_ld_b_hl(&mut self, bus: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        let hl = regs.hl();
        let data = bus.read(hl);
        *regs.b_mut() = data;
        8
    }

    fn inst_0x4e_ld_c_hl(&mut self, bus: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        let hl = regs.hl();
        let data = bus.read(hl);
        *regs.c_mut() = data;
        8
    }

    fn inst_0x56_ld_d_hl(&mut self, bus: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        let hl = regs.hl();
        let data = bus.read(hl);
        *regs.d_mut() = data;
        8
    }

    fn inst_0x5e_ld_e_hl(&mut self, bus: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        let hl = regs.hl();
        let data = bus.read(hl);
        *regs.e_mut() = data;
        8
    }

    fn inst_0x66_ld_h_hl(&mut self, bus: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        let hl = regs.hl();
        let data = bus.read(hl);
        *regs.h_mut() = data;
        8
    }

    fn inst_0x6e_ld_l_hl(&mut self, bus: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        let hl = regs.hl();
        let data = bus.read(hl);
        *regs.l_mut() = data;
        8
    }

    fn inst_0x7e_ld_a_hl(&mut self, bus: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        let hl = regs.hl();
        let data = bus.read(hl);
        *regs.a_mut() = data;
        8
    }
}

/// LD from 8bit register to memory instructions
/// LD (16 bits register pointers to memory), src
impl CPU {
    fn inst_0x02_ld_bc_a(&mut self, bus: &mut Bus) -> ClockCycle {
        let regs = self.regs();
        let bc = regs.bc();
        let data = regs.a();
        bus.write(bc, data);
        8
    }

    fn inst_0x12_ld_de_a(&mut self, bus: &mut Bus) -> ClockCycle {
        let regs = self.regs();
        let de = regs.de();
        let data = regs.a();
        bus.write(de, data);
        8
    }

    fn inst_0x70_ld_hl_b(&mut self, bus: &mut Bus) -> ClockCycle {
        let regs = self.regs();
        let hl = regs.hl();
        let data = regs.b();
        bus.write(hl, data);
        8
    }

    fn inst_0x71_ld_hl_c(&mut self, bus: &mut Bus) -> ClockCycle {
        let regs = self.regs();
        let hl = regs.hl();
        let data = regs.c();
        bus.write(hl, data);
        8
    }

    fn inst_0x72_ld_hl_d(&mut self, bus: &mut Bus) -> ClockCycle {
        let regs = self.regs();
        let hl = regs.hl();
        let data = regs.d();
        bus.write(hl, data);
        8
    }

    fn inst_0x73_ld_hl_e(&mut self, bus: &mut Bus) -> ClockCycle {
        let regs = self.regs();
        let hl = regs.hl();
        let data = regs.e();
        bus.write(hl, data);
        8
    }

    fn inst_0x74_ld_hl_h(&mut self, bus: &mut Bus) -> ClockCycle {
        let regs = self.regs();
        let hl = regs.hl();
        let data = regs.h();
        bus.write(hl, data);
        8
    }

    fn inst_0x75_ld_hl_l(&mut self, bus: &mut Bus) -> ClockCycle {
        let regs = self.regs();
        let hl = regs.hl();
        let data = regs.l();
        bus.write(hl, data);
        8
    }

    fn inst_0x77_ld_hl_a(&mut self, bus: &mut Bus) -> ClockCycle {
        let regs = self.regs();
        let hl = regs.hl();
        let data = regs.a();
        bus.write(hl, data);
        8
    }
}

/// special LD between 8bit registers and memory instructions
impl CPU {
    /// LD (HL+), A
    fn inst_0x22_ldi_hl_a(&mut self, bus: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        let hl = regs.hl();
        let data = regs.a();
        bus.write(hl, data);
        *regs.hl_mut() = regs.hl_mut().wrapping_add(1);
        8
    }

    /// LD A, (HL+)
    fn inst_0x2a_ldi_a_hl(&mut self, bus: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        let hl = regs.hl();
        let data = bus.read(hl);
        *regs.a_mut() = data;
        *regs.hl_mut() = regs.hl_mut().wrapping_add(1);
        8
    }

    /// LD (HL-), A
    fn inst_0x32_ldd_hl_a(&mut self, bus: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        let hl = regs.hl();
        let data = regs.a();
        bus.write(hl, data);
        *regs.hl_mut() = regs.hl_mut().wrapping_sub(1);
        8
    }

    /// LD A, (HL-)
    fn inst_0x3a_ldd_a_hl(&mut self, bus: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        let hl = regs.hl();
        let data = bus.read(hl);
        *regs.a_mut() = data;
        *regs.hl_mut() = regs.hl_mut().wrapping_sub(1);
        8
    }

    /// LD (0xFF00 + C), A
    fn inst_0xe2_ld_ff00_c_a(&mut self, bus: &mut Bus) -> ClockCycle {
        let regs = self.regs();
        let addr = IO_LOW_BOUND + regs.c() as Addr;
        let data = regs.a();
        bus.write(addr, data);
        8
    }

    /// LD A, (0xFF00 + C)
    fn inst_0xf2_ld_a_ff00_c(&mut self, bus: &mut Bus) -> ClockCycle {
        let regs = self.regs_mut();
        let addr = IO_LOW_BOUND + regs.c() as Addr;
        let data = bus.read(addr);
        *regs.a_mut() = data;
        8
    }
}

/// LD from immediate 8bit data to 8bit register instructions
impl CPU {
    fn read_imm8bit(&mut self, bus: &mut Bus) -> Word {
        let pc = self.pc();
        let data = bus.read(pc);
        self.pc_inc();
        data
    }

    fn inst_0x06_ld_b_imm8(&mut self, bus: &mut Bus) -> ClockCycle {
        let data = self.read_imm8bit(bus);
        *self.regs_mut().b_mut() = data;
        8
    }

    fn inst_0x0e_ld_c_imm8(&mut self, bus: &mut Bus) -> ClockCycle {
        let data = self.read_imm8bit(bus);
        *self.regs_mut().c_mut() = data;
        8
    }

    fn inst_0x16_ld_d_imm8(&mut self, bus: &mut Bus) -> ClockCycle {
        let data = self.read_imm8bit(bus);
        *self.regs_mut().d_mut() = data;
        8
    }

    fn inst_0x1e_ld_e_imm8(&mut self, bus: &mut Bus) -> ClockCycle {
        let data = self.read_imm8bit(bus);
        *self.regs_mut().e_mut() = data;
        8
    }

    fn inst_0x26_ld_h_imm8(&mut self, bus: &mut Bus) -> ClockCycle {
        let data = self.read_imm8bit(bus);
        *self.regs_mut().h_mut() = data;
        8
    }

    fn inst_0x2e_ld_l_imm8(&mut self, bus: &mut Bus) -> ClockCycle {
        let data = self.read_imm8bit(bus);
        *self.regs_mut().l_mut() = data;
        8
    }

    fn inst_0x3e_ld_a_imm8(&mut self, bus: &mut Bus) -> ClockCycle {
        let data = self.read_imm8bit(bus);
        *self.regs_mut().a_mut() = data;
        8
    }
}
