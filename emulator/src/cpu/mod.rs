use log::error;

use crate::{
    dev::bus::{Bus, IO_LOW_BOUND},
    error::{EmulatorError, Result},
    types::{Addr, ClockCycle, DWord, OpCode, Word},
};

use self::regs::Regs;

type Inst = fn(&mut CPU, &mut Bus) -> Result<ClockCycle>;

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
    pub fn clock(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let opcode = self.fetch_opcode(bus)?;
        self.pc_inc();
        let inst = Self::decode_inst(opcode);
        self.exec_inst(bus, inst)
    }

    fn fetch_opcode(&self, bus: &Bus) -> Result<OpCode> {
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

    fn exec_inst(&mut self, bus: &mut Bus, inst: Inst) -> Result<ClockCycle> {
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

    fn jr(&mut self, offset: Word) {
        let pc = self.regs().pc_mut();
        let offset = offset as i8 as i16 as Addr;
        *pc = pc.wrapping_add(offset)
    }

    fn enable_int(&mut self, bus: &mut Bus) {
        todo!()
    }
}

/// ref https://gbdev.io/pandocs/CPU_Instruction_Set.html
/// ref https://gbdev.io/gb-opcodes/optables/
/// ref https://archive.org/details/GameBoyProgManVer1.1/page/n93/mode/2up
/// Misc / control instructions
impl CPU {
    fn inst_0x00_nop(_: &mut CPU, _: &mut Bus) -> Result<ClockCycle> {
        Ok(4)
    }

    fn illegal_opcode(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let addr = self.pc() - 1;
        let opcode = bus.read(addr)?;
        error!("illegal opcode: 0x{opcode:02X} at address: 0x{addr:04X}",);
        Err(EmulatorError::IllegalInstruction)
    }
}

/// LD between 8bit registers instructions
/// LD dest, src
impl CPU {
    fn inst_0x40_ld_b_b(_: &mut CPU, _: &mut Bus) -> Result<ClockCycle> {
        Ok(4)
    }

    fn inst_0x41_ld_b_c(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.b_mut() = regs.c();
        Ok(4)
    }

    fn inst_0x42_ld_b_d(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.b_mut() = regs.d();
        Ok(4)
    }

    fn inst_0x43_ld_b_e(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.b_mut() = regs.e();
        Ok(4)
    }

    fn inst_0x44_ld_b_h(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.b_mut() = regs.h();
        Ok(4)
    }

    fn inst_0x45_ld_b_l(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.b_mut() = regs.l();
        Ok(4)
    }

    fn inst_0x47_ld_b_a(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.b_mut() = regs.a();
        Ok(4)
    }

    fn inst_0x48_ld_c_b(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.c_mut() = regs.b();
        Ok(4)
    }

    fn inst_0x49_ld_c_c(_: &mut CPU, _: &mut Bus) -> Result<ClockCycle> {
        Ok(4)
    }

    fn inst_0x4a_ld_c_d(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.c_mut() = regs.d();
        Ok(4)
    }

    fn inst_0x4b_ld_c_e(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.c_mut() = regs.e();
        Ok(4)
    }

    fn inst_0x4c_ld_c_h(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.c_mut() = regs.h();
        Ok(4)
    }

    fn inst_0x4d_ld_c_l(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.c_mut() = regs.l();
        Ok(4)
    }

    fn inst_0x4f_ld_c_a(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.c_mut() = regs.a();
        Ok(4)
    }

    fn inst_0x50_ld_d_b(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.d_mut() = regs.b();
        Ok(4)
    }

    fn inst_0x51_ld_d_c(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.d_mut() = regs.c();
        Ok(4)
    }

    fn inst_0x52_ld_d_d(_: &mut CPU, _: &mut Bus) -> Result<ClockCycle> {
        Ok(4)
    }

    fn inst_0x53_ld_d_e(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.d_mut() = regs.e();
        Ok(4)
    }

    fn inst_0x54_ld_d_h(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.d_mut() = regs.h();
        Ok(4)
    }

    fn inst_0x55_ld_d_l(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.d_mut() = regs.l();
        Ok(4)
    }

    fn inst_0x57_ld_d_a(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.d_mut() = regs.a();
        Ok(4)
    }

    fn inst_0x58_ld_e_b(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.e_mut() = regs.b();
        Ok(4)
    }

    fn inst_0x59_ld_e_c(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.e_mut() = regs.c();
        Ok(4)
    }

    fn inst_0x5a_ld_e_d(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.e_mut() = regs.d();
        Ok(4)
    }

    fn inst_0x5b_ld_e_e(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        Ok(4)
    }

    fn inst_0x5c_ld_e_h(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.e_mut() = regs.h();
        Ok(4)
    }

    fn inst_0x5d_ld_e_l(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.e_mut() = regs.l();
        Ok(4)
    }

    fn inst_0x5f_ld_e_a(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.e_mut() = regs.a();
        Ok(4)
    }

    fn inst_0x60_ld_h_b(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.h_mut() = regs.b();
        Ok(4)
    }

    fn inst_0x61_ld_h_c(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.h_mut() = regs.c();
        Ok(4)
    }

    fn inst_0x62_ld_h_d(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.h_mut() = regs.d();
        Ok(4)
    }

    fn inst_0x63_ld_h_e(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.h_mut() = regs.e();
        Ok(4)
    }

    fn inst_0x64_ld_h_h(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        Ok(4)
    }

    fn inst_0x65_ld_h_l(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.h_mut() = regs.l();
        Ok(4)
    }

    fn inst_0x67_ld_h_a(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.h_mut() = regs.a();
        Ok(4)
    }

    fn inst_0x68_ld_l_b(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.l_mut() = regs.b();
        Ok(4)
    }

    fn inst_0x69_ld_l_c(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.l_mut() = regs.c();
        Ok(4)
    }

    fn inst_0x6a_ld_l_d(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.l_mut() = regs.d();
        Ok(4)
    }

    fn inst_0x6b_ld_l_e(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.l_mut() = regs.e();
        Ok(4)
    }

    fn inst_0x6c_ld_l_h(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.l_mut() = regs.h();
        Ok(4)
    }

    fn inst_0x6d_ld_l_l(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        Ok(4)
    }

    fn inst_0x6f_ld_l_a(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.l_mut() = regs.a();
        Ok(4)
    }

    fn inst_0x78_ld_a_b(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.a_mut() = regs.b();
        Ok(4)
    }

    fn inst_0x79_ld_a_c(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.a_mut() = regs.c();
        Ok(4)
    }

    fn inst_0x7a_ld_a_d(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.a_mut() = regs.d();
        Ok(4)
    }

    fn inst_0x7b_ld_a_e(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.a_mut() = regs.e();
        Ok(4)
    }

    fn inst_0x7c_ld_a_h(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.a_mut() = regs.h();
        Ok(4)
    }

    fn inst_0x7d_ld_a_l(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        *regs.a_mut() = regs.l();
        Ok(4)
    }

    fn inst_0x7f_ld_a_a(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        Ok(4)
    }
}

/// LD from memory to 8bit register instructions
/// LD dest, (16 bits register pointers to memory)
impl CPU {
    fn inst_0x0a_ld_a_mbc(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        let bc = regs.bc();
        let data = bus.read(bc)?;
        *regs.a_mut() = data;
        Ok(8)
    }

    fn inst_0x1a_ld_a_mde(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        let de = regs.de();
        let data = bus.read(de)?;
        *regs.a_mut() = data;
        Ok(8)
    }

    fn inst_0x46_ld_b_mhl(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        let hl = regs.hl();
        let data = bus.read(hl)?;
        *regs.b_mut() = data;
        Ok(8)
    }

    fn inst_0x4e_ld_c_mhl(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        let hl = regs.hl();
        let data = bus.read(hl)?;
        *regs.c_mut() = data;
        Ok(8)
    }

    fn inst_0x56_ld_d_mhl(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        let hl = regs.hl();
        let data = bus.read(hl)?;
        *regs.d_mut() = data;
        Ok(8)
    }

    fn inst_0x5e_ld_e_mhl(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        let hl = regs.hl();
        let data = bus.read(hl)?;
        *regs.e_mut() = data;
        Ok(8)
    }

    fn inst_0x66_ld_h_mhl(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        let hl = regs.hl();
        let data = bus.read(hl)?;
        *regs.h_mut() = data;
        Ok(8)
    }

    fn inst_0x6e_ld_l_mhl(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        let hl = regs.hl();
        let data = bus.read(hl)?;
        *regs.l_mut() = data;
        Ok(8)
    }

    fn inst_0x7e_ld_a_mhl(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        let hl = regs.hl();
        let data = bus.read(hl)?;
        *regs.a_mut() = data;
        Ok(8)
    }
}

/// LD from 8bit register to memory instructions
/// LD (16 bits register pointers to memory), src
impl CPU {
    fn inst_0x02_ld_mbc_a(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs();
        let bc = regs.bc();
        let data = regs.a();
        bus.write(bc, data)?;
        Ok(8)
    }

    fn inst_0x12_ld_mde_a(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs();
        let de = regs.de();
        let data = regs.a();
        bus.write(de, data)?;
        Ok(8)
    }

    fn inst_0x70_ld_mhl_b(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs();
        let hl = regs.hl();
        let data = regs.b();
        bus.write(hl, data)?;
        Ok(8)
    }

    fn inst_0x71_ld_mhl_c(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs();
        let hl = regs.hl();
        let data = regs.c();
        bus.write(hl, data)?;
        Ok(8)
    }

    fn inst_0x72_ld_mhl_d(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs();
        let hl = regs.hl();
        let data = regs.d();
        bus.write(hl, data)?;
        Ok(8)
    }

    fn inst_0x73_ld_mhl_e(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs();
        let hl = regs.hl();
        let data = regs.e();
        bus.write(hl, data)?;
        Ok(8)
    }

    fn inst_0x74_ld_mhl_h(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs();
        let hl = regs.hl();
        let data = regs.h();
        bus.write(hl, data)?;
        Ok(8)
    }

    fn inst_0x75_ld_mhl_l(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs();
        let hl = regs.hl();
        let data = regs.l();
        bus.write(hl, data)?;
        Ok(8)
    }

    fn inst_0x77_ld_mhl_a(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs();
        let hl = regs.hl();
        let data = regs.a();
        bus.write(hl, data)?;
        Ok(8)
    }
}

/// special LD between 8bit registers and memory instructions
impl CPU {
    /// LD (HL+), A
    fn inst_0x22_ldi_mhl_a(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        let hl = regs.hl();
        let data = regs.a();
        bus.write(hl, data)?;
        *regs.hl_mut() = regs.hl_mut().wrapping_add(1);
        Ok(8)
    }

    /// LD A, (HL+)
    fn inst_0x2a_ldi_a_mhl(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        let hl = regs.hl();
        let data = bus.read(hl)?;
        *regs.a_mut() = data;
        *regs.hl_mut() = regs.hl_mut().wrapping_add(1);
        Ok(8)
    }

    /// LD (HL-), A
    fn inst_0x32_ldd_mhl_a(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        let hl = regs.hl();
        let data = regs.a();
        bus.write(hl, data)?;
        *regs.hl_mut() = regs.hl_mut().wrapping_sub(1);
        Ok(8)
    }

    /// LD A, (HL-)
    fn inst_0x3a_ldd_a_mhl(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        let hl = regs.hl();
        let data = bus.read(hl)?;
        *regs.a_mut() = data;
        *regs.hl_mut() = regs.hl_mut().wrapping_sub(1);
        Ok(8)
    }

    /// LDH (C), A
    fn inst_0xe2_ldh_mc_a(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs();
        let addr = IO_LOW_BOUND + regs.c() as Addr;
        let data = regs.a();
        bus.write(addr, data)?;
        Ok(8)
    }

    /// LDH A, (C)
    fn inst_0xf2_ldh_a_mc(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        let addr = IO_LOW_BOUND + regs.c() as Addr;
        let data = bus.read(addr)?;
        *regs.a_mut() = data;
        Ok(8)
    }
}

/// LD from immediate 8bit data to 8bit register instructions
impl CPU {
    #[inline]
    fn read_imm8bit(&mut self, bus: &mut Bus) -> Result<Word> {
        let pc = self.pc();
        let data = bus.read(pc)?;
        self.pc_inc();
        Ok(data)
    }

    fn inst_0x06_ld_b_imm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.read_imm8bit(bus)?;
        *self.regs_mut().b_mut() = data;
        Ok(8)
    }

    fn inst_0x0e_ld_c_imm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.read_imm8bit(bus)?;
        *self.regs_mut().c_mut() = data;
        Ok(8)
    }

    fn inst_0x16_ld_d_imm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.read_imm8bit(bus)?;
        *self.regs_mut().d_mut() = data;
        Ok(8)
    }

    fn inst_0x1e_ld_e_imm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.read_imm8bit(bus)?;
        *self.regs_mut().e_mut() = data;
        Ok(8)
    }

    fn inst_0x26_ld_h_imm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.read_imm8bit(bus)?;
        *self.regs_mut().h_mut() = data;
        Ok(8)
    }

    fn inst_0x2e_ld_l_imm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.read_imm8bit(bus)?;
        *self.regs_mut().l_mut() = data;
        Ok(8)
    }

    fn inst_0x3e_ld_a_imm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.read_imm8bit(bus)?;
        *self.regs_mut().a_mut() = data;
        Ok(8)
    }
}

/// LD (16 bits register pointers to memory), immediate 8bit data
impl CPU {
    fn inst_0x36_ld_mdl_imm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let word = self.read_imm8bit(bus)?;
        let hl = self.regs().hl();
        bus.write(hl, word)?;
        Ok(12)
    }
}

/// LDH between (0xFF00 + immediate 8bit data) and A instructions
impl CPU {
    fn inst_0xe0_ldh_mimm8_a(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let imme8 = self.read_imm8bit(bus)?;
        let addr = IO_LOW_BOUND + imme8 as Addr;
        let a = self.regs().a();
        bus.write(addr, a)?;
        Ok(12)
    }

    fn inst_0xf0_ldh_a_mimm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let imme8 = self.read_imm8bit(bus)?;
        let addr = IO_LOW_BOUND + imme8 as Addr;
        let data = bus.read(addr)?;
        *self.regs_mut().a_mut() = data;
        Ok(12)
    }
}

/// LD from 16bit immediate data to 16bit register instructions
impl CPU {
    #[inline]
    fn read_imm16bit(&mut self, bus: &mut Bus) -> Result<DWord> {
        let pc = self.pc();
        let low = bus.read(pc)?;
        let high = bus.read(pc + 1)?;
        self.pc_inc_by(2);
        let ret = (high as DWord) << 8 | low as DWord;
        Ok(ret)
    }

    fn inst_0x01_ld_bc_imm16(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.read_imm16bit(bus)?;
        *self.regs_mut().bc_mut() = data;
        Ok(12)
    }

    fn inst_0x11_ld_de_imm16(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.read_imm16bit(bus)?;
        *self.regs_mut().de_mut() = data;
        Ok(12)
    }

    fn inst_0x21_ld_hl_imm16(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.read_imm16bit(bus)?;
        *self.regs_mut().hl_mut() = data;
        Ok(12)
    }

    fn inst_0x31_ld_sp_imm16(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.read_imm16bit(bus)?;
        *self.regs_mut().sp_mut() = data;
        Ok(12)
    }
}

/// LD from SP to (16 bits register pointers to memory)
impl CPU {
    fn inst_0x08_ld_mimm16_sp(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let addr = self.read_imm16bit(bus)?;
        let sp = self.regs().sp();
        let low = (sp & 0xFF) as Word;
        let high = (sp >> 8) as Word;
        bus.write(addr, low);
        bus.write(addr + 1, high);
        Ok(20)
    }
}

/// LD from HL to SP instructions
impl CPU {
    fn inst_0xf9_ld_sp_hl(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs();
        *self.regs_mut().sp_mut() = regs.hl();
        Ok(8)
    }
}

/// LD between 16bit immediate pointers to memory and A
impl CPU {
    fn inst_0xea_ld_mimm16_a(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let addr = self.read_imm16bit(bus)?;
        let a = self.regs().a();
        bus.write(addr, a)?;
        Ok(16)
    }

    fn inst_0xfa_ld_a_mimm16(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let addr = self.read_imm16bit(bus)?;
        let data = bus.read(addr)?;
        *self.regs_mut().a_mut() = data;
        Ok(16)
    }
}

impl CPU {
    /// LD HL, SP + imme8
    fn inst_0xf8_ld_hl_sp_imm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let imme8 = self.read_imm8bit(bus)? as i8 as i16;
        let regs = self.regs_mut();
        regs.zero_flag_mut().clear();
        regs.negative_flag_mut().clear();
        let sp = regs.sp() as i16;
        let result = sp.wrapping_add(imme8);
        let check = sp ^ imme8 ^ result;
        regs.half_carry_flag_mut().set_value((check & 0x10) != 0);
        regs.carry_flag_mut().set_value((check & 0x100) != 0);
        Ok(12)
    }
}

/// CP
impl CPU {
    fn cp_a_with(&mut self, val: Word) {
        let regs = self.regs_mut();
        let a = regs.a();
        regs.zero_flag_mut().set_value(a == val);
        regs.negative_flag_mut().set();
        regs.half_carry_flag_mut()
            .set_value((a & 0xF) < (val & 0xF));
        regs.carry_flag_mut().set_value(a < val);
    }

    fn inst_0xb8_cp_b(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.cp_a_with(self.regs().b());
        Ok(4)
    }

    fn inst_0xb9_cp_c(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.cp_a_with(self.regs().c());
        Ok(4)
    }

    fn inst_0xba_cp_d(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.cp_a_with(self.regs().d());
        Ok(4)
    }

    fn inst_0xbb_cp_e(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.cp_a_with(self.regs().e());
        Ok(4)
    }

    fn inst_0xbc_cp_h(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.cp_a_with(self.regs().h());
        Ok(4)
    }

    fn inst_0xbd_cp_l(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.cp_a_with(self.regs().l());
        Ok(4)
    }

    fn inst_0xbf_cp_a(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.cp_a_with(self.regs().a());
        Ok(4)
    }

    /// CP (HL)
    fn inst_0xbe_cp_mhl(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let hl = self.regs().hl();
        let data = bus.read(hl)?;
        self.cp_a_with(data);
        Ok(8)
    }

    /// CP imme8
    fn inst_0xfe_cp_imm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.read_imm8bit(bus)?;
        self.cp_a_with(data);
        Ok(8)
    }
}

/// JP & JR
impl CPU {
    /// JP imme16
    fn inst_0xc3_jp_imme16(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let addr = self.read_imm16bit(bus)?;
        self.jp(addr);
        Ok(16)
    }

    /// JP NZ, imme16
    fn inst_0xc2_jp_nz_imme16(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let jump_to = self.read_imm16bit(bus)?;
        if !self.regs.zero_flag() {
            self.jp(jump_to);
            Ok(16)
        } else {
            Ok(12)
        }
    }

    /// JP Z, imme16
    fn inst_0xca_jp_z_imm16(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let jump_to = self.read_imm16bit(bus)?;
        if self.regs.zero_flag() {
            self.jp(jump_to);
            Ok(16)
        } else {
            Ok(12)
        }
    }

    /// JP NC, imme16
    fn inst_0xd1_jp_nc_imme16(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let jump_to = self.read_imm16bit(bus)?;
        if !self.regs.carry_flag() {
            self.jp(jump_to);
            Ok(16)
        } else {
            Ok(12)
        }
    }

    /// JP C, imme16
    fn inst_0xda_jp_c_imme16(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let jump_to = self.read_imm16bit(bus)?;
        if self.regs.carry_flag() {
            self.jp(jump_to);
            Ok(16)
        } else {
            Ok(12)
        }
    }

    /// JP HL
    fn inst_0xe9_jp_hl(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let to = self.regs().hl();
        self.jp(to);
        // 只花费1个机器周期，没有流水线停顿的惩罚
        Ok(4)
    }

    /// JR imme8
    fn inst_0x18_jr_imm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let offset = self.read_imm8bit(bus)?;
        self.jr(offset);
        Ok(12)
    }

    /// JR NZ, imme8
    fn inst_0x20_jr_nz_imm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let offset = self.read_imm8bit(bus)?;
        if !self.regs.zero_flag() {
            self.jr(offset);
            Ok(12)
        } else {
            Ok(8)
        }
    }

    /// JR Z, imme8
    fn inst_0x28_jr_z_imm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let offset = self.read_imm8bit(bus)?;
        if self.regs.zero_flag() {
            self.jr(offset);
            Ok(12)
        } else {
            Ok(8)
        }
    }

    /// JR NC, imme8
    fn inst_0x30_jr_nc_imm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let offset = self.read_imm8bit(bus)?;
        if !self.regs.carry_flag() {
            self.jr(offset);
            Ok(12)
        } else {
            Ok(8)
        }
    }

    /// JR C, imme8
    fn inst_0x38_jr_c_imm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let offset = self.read_imm8bit(bus)?;
        if self.regs.carry_flag() {
            self.jr(offset);
            Ok(12)
        } else {
            Ok(8)
        }
    }
}

/// PUSH & POP & RET & CALL
impl CPU {
    fn push_dword(&mut self, bus: &mut Bus, data: DWord) -> Result {
        let sp = self.regs.sp_mut();
        *sp -= 2;
        let low = (data & 0xFF) as Word;
        let high = (data >> 8) as Word;
        bus.write(*sp, low)?;
        bus.write(*sp + 1, high)?;
        Ok(())
    }

    /// PUSH BC
    fn inst_0xc5_push_bc(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.regs().bc();
        self.push_dword(bus, data)?;
        Ok(16)
    }

    /// PUSH DE
    fn inst_0xd5_push_de(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.regs().de();
        self.push_dword(bus, data)?;
        Ok(16)
    }

    /// PUSH HL
    fn inst_0xe5_push_hl(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.regs().hl();
        self.push_dword(bus, data)?;
        Ok(16)
    }

    /// PUSH AF
    fn inst_0xf5_push_af(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.regs().af();
        self.push_dword(bus, data)?;
        Ok(16)
    }

    fn pop_dword(&mut self, bus: &mut Bus) -> Result<DWord> {
        let sp = self.regs.sp_mut();
        let low = bus.read(*sp)?;
        let high = bus.read(*sp + 1)?;
        *sp += 2;
        let ret = (high as DWord) << 8 | low as DWord;
        Ok(ret)
    }

    /// POP BC
    fn inst_0xc1_pop_bc(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.pop_dword(bus)?;
        *self.regs_mut().bc_mut() = data;
        Ok(12)
    }

    /// POP DE
    fn inst_0xd1_pop_de(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.pop_dword(bus)?;
        *self.regs_mut().de_mut() = data;
        Ok(12)
    }

    /// POP HL
    fn inst_0xe1_pop_hl(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.pop_dword(bus)?;
        *self.regs_mut().hl_mut() = data;
        Ok(12)
    }

    /// POP AF
    fn inst_0xf1_pop_af(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.pop_dword(bus)?;
        *self.regs_mut().af_mut() = data;
        Ok(12)
    }

    /// CALL imme16
    fn inst_0xcd_call_imm16(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let addr = self.read_imm16bit(bus)?;
        let pc = self.pc();
        self.push_dword(bus, pc)?;
        self.jp(addr);
        Ok(24)
    }

    /// CALL NZ, imme16
    fn inst_0xc4_call_nz_imm16(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let jump_to = self.read_imm16bit(bus)?;
        if !self.regs.zero_flag() {
            let pc = self.pc();
            self.push_dword(bus, pc)?;
            self.jp(jump_to);
            Ok(24)
        } else {
            Ok(12)
        }
    }

    /// CALL Z, imme16
    fn inst_0xcc_call_z_imm16(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let jump_to = self.read_imm16bit(bus)?;
        if self.regs.zero_flag() {
            let pc = self.pc();
            self.push_dword(bus, pc)?;
            self.jp(jump_to);
            Ok(24)
        } else {
            Ok(12)
        }
    }

    /// CALL NC, imme16
    fn inst_0xd4_call_nc_imm16(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let jump_to = self.read_imm16bit(bus)?;
        if !self.regs.carry_flag() {
            let pc = self.pc();
            self.push_dword(bus, pc)?;
            self.jp(jump_to);
            Ok(24)
        } else {
            Ok(12)
        }
    }

    /// CALL C, imme16
    fn inst_0xdc_call_c_imm16(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let jump_to = self.read_imm16bit(bus)?;
        if self.regs.carry_flag() {
            let pc = self.pc();
            self.push_dword(bus, pc)?;
            self.jp(jump_to);
            Ok(24)
        } else {
            Ok(12)
        }
    }

    /// RET
    fn inst_0xc9_ret(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let addr = self.pop_dword(bus)?;
        self.jp(addr);
        Ok(16)
    }

    /// RET NZ
    fn inst_0xc0_ret_nz(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        if !self.regs.zero_flag() {
            let addr = self.pop_dword(bus)?;
            self.jp(addr);
            Ok(20)
        } else {
            Ok(8)
        }
    }

    /// RET Z
    fn inst_0xc8_ret_z(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        if self.regs.zero_flag() {
            let addr = self.pop_dword(bus)?;
            self.jp(addr);
            Ok(20)
        } else {
            Ok(8)
        }
    }

    /// RET NC
    fn inst_0xd0_ret_nc(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        if !self.regs.carry_flag() {
            let addr = self.pop_dword(bus)?;
            self.jp(addr);
            Ok(20)
        } else {
            Ok(8)
        }
    }

    /// RET C
    fn inst_0xd8_ret_c(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        if self.regs.carry_flag() {
            let addr = self.pop_dword(bus)?;
            self.jp(addr);
            Ok(20)
        } else {
            Ok(8)
        }
    }

    /// RETI
    fn inst_0xd9_reti(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let addr = self.pop_dword(bus)?;
        self.jp(addr);
        self.enable_int(bus);
        Ok(16)
    }

    /// RST 0x0000
    fn inst_0xc7_rst_0x0000(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        self.push_dword(bus, self.pc());
        self.jp(0x0000);
        Ok(16)
    }

    /// RST 0x0008
    fn inst_0xcf_rst_0x0008(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        self.push_dword(bus, self.pc());
        self.jp(0x0008);
        Ok(16)
    }

    /// RST 0x0010
    fn inst_0xd7_rst_0x0010(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        self.push_dword(bus, self.pc());
        self.jp(0x0010);
        Ok(16)
    }

    /// RST 0x0018
    fn inst_0xdf_rst_0x0018(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        self.push_dword(bus, self.pc());
        self.jp(0x0018);
        Ok(16)
    }

    /// RST 0x0020
    fn inst_0xe7_rst_0x0020(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        self.push_dword(bus, self.pc());
        self.jp(0x0020);
        Ok(16)
    }

    /// RST 0x0028
    fn inst_0xef_rst_0x0028(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        self.push_dword(bus, self.pc());
        self.jp(0x0028);
        Ok(16)
    }

    /// RST 0x0030
    fn inst_0xf7_rst_0x0030(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        self.push_dword(bus, self.pc());
        self.jp(0x0030);
        Ok(16)
    }

    /// RST 0x0038
    fn inst_0xff_rst_0x0038(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        self.push_dword(bus, self.pc());
        self.jp(0x0038);
        Ok(16)
    }
}
