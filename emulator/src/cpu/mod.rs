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
        let pc = self.regs_mut().pc_mut();
        let offset = offset as i8 as i16 as Addr;
        *pc = pc.wrapping_add(offset)
    }

    fn enable_int(&mut self, _: &mut Bus) {
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
        regs.half_carry_flag_mut().set_value(check & 0x10 != 0);
        regs.carry_flag_mut().set_value(check & 0x100 != 0);
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

/// 算术逻辑运算指令
impl CPU {
    /// INC B
    fn inst_0x04_inc_b(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        let b = regs.b_mut();
        let result = b.wrapping_add(1);
        *b = result;
        regs.zero_flag_mut().set_value(result == 0);
        regs.negative_flag_mut().clear();
        regs.half_carry_flag_mut().set_value((result & 0xF) == 0x0);
        Ok(4)
    }

    /// DEC B
    fn inst_0x05_dec_b(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        let b = regs.b_mut();
        let result = b.wrapping_sub(1);
        *b = result;
        regs.zero_flag_mut().set_value(result == 0);
        regs.negative_flag_mut().clear();
        regs.half_carry_flag_mut().set_value((result & 0xF) == 0xF);
        Ok(4)
    }

    /// INC C
    fn inst_0x0c_inc_c(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        let c = regs.c_mut();
        let result = c.wrapping_add(1);
        *c = result;
        regs.zero_flag_mut().set_value(result == 0);
        regs.negative_flag_mut().clear();
        regs.half_carry_flag_mut().set_value((result & 0xF) == 0x0);
        Ok(4)
    }

    /// DEC C
    fn inst_0x0d_dec_c(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        let c = regs.c_mut();
        let result = c.wrapping_sub(1);
        *c = result;
        regs.zero_flag_mut().set_value(result == 0);
        regs.negative_flag_mut().clear();
        regs.half_carry_flag_mut().set_value((result & 0xF) == 0xF);
        Ok(4)
    }

    /// INC D
    fn inst_0x14_inc_d(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        let d = regs.d_mut();
        let result = d.wrapping_add(1);
        *d = result;
        regs.zero_flag_mut().set_value(result == 0);
        regs.negative_flag_mut().clear();
        regs.half_carry_flag_mut().set_value((result & 0xF) == 0x0);
        Ok(4)
    }

    /// DEC D
    fn inst_0x15_dec_d(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        let d = regs.d_mut();
        let result = d.wrapping_sub(1);
        *d = result;
        regs.zero_flag_mut().set_value(result == 0);
        regs.negative_flag_mut().clear();
        regs.half_carry_flag_mut().set_value((result & 0xF) == 0xF);
        Ok(4)
    }

    /// INC E
    fn inst_0x1c_inc_e(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        let e = regs.e_mut();
        let result = e.wrapping_add(1);
        *e = result;
        regs.zero_flag_mut().set_value(result == 0);
        regs.negative_flag_mut().clear();
        regs.half_carry_flag_mut().set_value((result & 0xF) == 0x0);
        Ok(4)
    }

    /// DEC E
    fn inst_0x1d_dec_e(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        let e = regs.e_mut();
        let result = e.wrapping_sub(1);
        *e = result;
        regs.zero_flag_mut().set_value(result == 0);
        regs.negative_flag_mut().clear();
        regs.half_carry_flag_mut().set_value((result & 0xF) == 0xF);
        Ok(4)
    }

    /// INC H
    fn inst_0x24_inc_h(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        let h = regs.h_mut();
        let result = h.wrapping_add(1);
        *h = result;
        regs.zero_flag_mut().set_value(result == 0);
        regs.negative_flag_mut().clear();
        regs.half_carry_flag_mut().set_value((result & 0xF) == 0x0);
        Ok(4)
    }

    /// DEC HINC
    fn inst_0x25_dec_h(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        let h = regs.h_mut();
        let result = h.wrapping_sub(1);
        *h = result;
        regs.zero_flag_mut().set_value(result == 0);
        regs.negative_flag_mut().clear();
        regs.half_carry_flag_mut().set_value((result & 0xF) == 0xF);
        Ok(4)
    }

    /// INC L
    fn inst_0x2c_inc_l(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        let l = regs.l_mut();
        let result = l.wrapping_add(1);
        *l = result;
        regs.zero_flag_mut().set_value(result == 0);
        regs.negative_flag_mut().clear();
        regs.half_carry_flag_mut().set_value((result & 0xF) == 0x0);
        Ok(4)
    }

    /// DEC L
    fn inst_0x2d_dec_l(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        let l = regs.l_mut();
        let result = l.wrapping_sub(1);
        *l = result;
        regs.zero_flag_mut().set_value(result == 0);
        regs.negative_flag_mut().clear();
        regs.half_carry_flag_mut().set_value((result & 0xF) == 0xF);
        Ok(4)
    }

    /// INC A
    fn inst_0x3c_inc_a(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        let a = regs.a_mut();
        let result = a.wrapping_add(1);
        *a = result;
        regs.zero_flag_mut().set_value(result == 0);
        regs.negative_flag_mut().clear();
        regs.half_carry_flag_mut().set_value((result & 0xF) == 0x0);
        Ok(4)
    }

    /// DEC A
    fn inst_0x3d_dec_a(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        let a = regs.a_mut();
        let result = a.wrapping_sub(1);
        *a = result;
        regs.zero_flag_mut().set_value(result == 0);
        regs.negative_flag_mut().clear();
        regs.half_carry_flag_mut().set_value((result & 0xF) == 0xF);
        Ok(4)
    }

    /// INC (HL)
    fn inst_0x34_inc_mhl(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        let addr = regs.hl();
        let data = bus.read(addr)? + 1;
        regs.zero_flag_mut().set_value(data == 0);
        regs.negative_flag_mut().clear();
        regs.half_carry_flag_mut().set_value((data & 0xF) == 0x0);
        bus.write(addr, data)?;
        Ok(12)
    }

    /// DEC (HL)
    fn inst_0x35_dec_mhl(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let regs = self.regs_mut();
        let addr = regs.hl();
        let data = bus.read(addr)? - 1;
        regs.zero_flag_mut().set_value(data == 0);
        regs.negative_flag_mut().clear();
        regs.half_carry_flag_mut().set_value((data & 0xF) == 0xF);
        bus.write(addr, data)?;
        Ok(12)
    }

    /// INC BC
    fn inst_0x03_inc_bc(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let bc = self.regs_mut().bc_mut();
        *bc = bc.wrapping_add(1);
        Ok(8)
    }

    /// DEC BC
    fn inst_0x0b_dec_bc(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let bc = self.regs_mut().bc_mut();
        *bc = bc.wrapping_sub(1);
        Ok(8)
    }

    /// INC DE
    fn inst_0x13_inc_de(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let de = self.regs_mut().de_mut();
        *de = de.wrapping_add(1);
        Ok(8)
    }

    /// DEC DE
    fn inst_0x1b_dec_de(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let de = self.regs_mut().de_mut();
        *de = de.wrapping_sub(1);
        Ok(8)
    }

    /// INC HL
    fn inst_0x23_inc_hl(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let hl = self.regs_mut().hl_mut();
        *hl = hl.wrapping_add(1);
        Ok(8)
    }

    /// DEC HL
    fn inst_0x2b_dec_hl(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let hl = self.regs_mut().hl_mut();
        *hl = hl.wrapping_sub(1);
        Ok(8)
    }

    /// INC SP
    fn inst_0x33_inc_sp(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let sp = self.regs_mut().sp_mut();
        *sp = sp.wrapping_add(1);
        Ok(8)
    }

    /// DEC SP
    fn inst_0x3b_dec_sp(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let sp = self.regs_mut().sp_mut();
        *sp = sp.wrapping_sub(1);
        Ok(8)
    }

    fn add_a_with(&mut self, rhs: Word) {
        let regs = self.regs_mut();
        let lhs = regs.a() as u32;
        let rhs = rhs as u32;
        let result = lhs.wrapping_add(rhs);
        regs.negative_flag_mut().clear();
        regs.zero_flag_mut().set_value(result & 0xFF == 0);
        regs.half_carry_flag_mut()
            .set_value(lhs & 0xF + rhs & 0xF > 0xF);
        regs.carry_flag_mut().set_value(result > 0xFF);
        *regs.a_mut() = result as Word;
    }

    /// ADD A, B
    fn inst_0x80_add_a_b(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.add_a_with(self.regs().b());
        Ok(4)
    }

    /// ADD A, C
    fn inst_0x81_add_a_c(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.add_a_with(self.regs().c());
        Ok(4)
    }

    /// ADD A, D
    fn inst_0x82_add_a_d(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.add_a_with(self.regs().d());
        Ok(4)
    }

    /// ADD A, E
    fn inst_0x83_add_a_e(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.add_a_with(self.regs().e());
        Ok(4)
    }

    /// ADD A, H
    fn inst_0x84_add_a_h(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.add_a_with(self.regs().h());
        Ok(4)
    }

    /// ADD A, L
    fn inst_0x85_add_a_l(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.add_a_with(self.regs().l());
        Ok(4)
    }

    /// ADD A, A
    fn inst_0x87_add_a_a(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.add_a_with(self.regs().a());
        Ok(4)
    }

    /// ADD A, imm8
    fn inst_0xc6_add_a_imm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.read_imm8bit(bus)?;
        self.add_a_with(data);
        Ok(8)
    }

    /// ADD A, (HL)
    fn inst_0x86_add_a_mhl(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = bus.read(self.regs().hl())?;
        self.add_a_with(data);
        Ok(8)
    }

    fn add_hl_with(&mut self, rhs: DWord) {
        let regs = self.regs_mut();
        let lhs = regs.hl() as u32;
        let rhs = rhs as u32;
        let result = lhs.wrapping_add(rhs);
        regs.negative_flag_mut().clear();
        regs.half_carry_flag_mut()
            .set_value(lhs & 0xFFF + rhs & 0xFFF > 0xFFF);
        regs.carry_flag_mut().set_value(result > 0xFFFF);
        *regs.hl_mut() = result as DWord;
    }

    /// ADD HL, BC
    fn inst_0x09_add_hl_bc(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.add_hl_with(self.bc());
        Ok(8)
    }

    /// ADD HL, DE
    fn inst_0x19_add_hl_de(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.add_hl_with(self.de());
        Ok(8)
    }

    /// ADD HL, DE
    fn inst_0x29_add_hl_hl(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.add_hl_with(self.hl());
        Ok(8)
    }

    /// ADD HL, SP
    fn inst_0x39_add_hl_sp(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.add_hl_with(self.sp());
        Ok(8)
    }

    /// ADD SP, imm8
    fn inst_0xe8_add_sp_imm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let lhs = self.sp();
        let imm8 = self.read_imm8bit(bus)? as i8 as DWord;
        let result = lhs.wrapping_add(imm8);
        let check = lhs ^ result ^ imm8;
        *self.sp_mut() = result;
        self.zero_flag_mut().clear();
        self.negative_flag_mut().clear();
        self.half_carry_flag_mut().set_value(check & 0x10 != 0);
        self.carry_flag_mut().set_value(check & 0x100 != 0);
        Ok(16)
    }

    fn adc_a_with(&mut self, rhs: Word) {
        let lhs = self.a() as u32;
        let rhs = rhs as u32;
        let carry: u32 = if self.carry_flag() { 1 } else { 0 };
        let result = lhs + rhs + carry;
        self.zero_flag_mut().set_value(result & 0xFF == 0);
        self.negative_flag_mut().clear();
        self.half_carry_flag_mut()
            .set_value(lhs & 0xF + rhs & 0xF + carry > 0xF);
        self.carry_flag_mut().set_value(result > 0xFF);
        *self.a_mut() = result as Word;
    }

    fn inst_0x88_adc_a_b(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.adc_a_with(self.b());
        Ok(4)
    }

    fn inst_0x89_adc_a_c(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.adc_a_with(self.c());
        Ok(4)
    }

    fn inst_0x8a_adc_a_d(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.adc_a_with(self.d());
        Ok(4)
    }

    fn inst_0x8b_adc_a_e(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.adc_a_with(self.e());
        Ok(4)
    }

    fn inst_0x8c_adc_a_h(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.adc_a_with(self.h());
        Ok(4)
    }

    fn inst_0x8d_adc_a_l(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.adc_a_with(self.l());
        Ok(4)
    }

    fn inst_0x8e_adc_a_a(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.adc_a_with(self.a());
        Ok(4)
    }

    /// ADC A, imm8
    fn inst_0xce_add_a_imm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.read_imm8bit(bus)?;
        self.adc_a_with(data);
        Ok(8)
    }

    /// ADC A, (HL)
    fn inst_0x8e_adc_a_mhl(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = bus.read(self.hl())?;
        self.adc_a_with(data);
        Ok(8)
    }

    fn sub_a_with(&mut self, rhs: Word) {
        let lhs = self.a();
        let result = lhs.wrapping_sub(rhs);
        self.zero_flag_mut().set_value(result == 0);
        self.negative_flag_mut().set();
        self.half_carry_flag_mut().set_value(lhs & 0xF < rhs & 0xF);
        self.carry_flag_mut().set_value(lhs < rhs);
        *self.a_mut() = result;
    }

    fn inst_0x90_sub_a_b(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.sub_a_with(self.b());
        Ok(4)
    }

    fn inst_0x91_sub_a_c(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.sub_a_with(self.c());
        Ok(4)
    }

    fn inst_0x92_sub_a_d(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.sub_a_with(self.d());
        Ok(4)
    }

    fn inst_0x93_sub_a_e(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.sub_a_with(self.e());
        Ok(4)
    }

    fn inst_0x94_sub_a_h(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.sub_a_with(self.h());
        Ok(4)
    }

    fn inst_0x95_sub_a_l(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.sub_a_with(self.l());
        Ok(4)
    }

    fn inst_0x96_sub_a_a(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.sub_a_with(self.a());
        Ok(4)
    }

    /// SUB A, imm8
    fn inst_0xd6_sub_a_imm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.read_imm8bit(bus)?;
        self.sub_a_with(data);
        Ok(8)
    }

    /// SUB A, (HL)
    fn inst_0x96_sub_a_mhl(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = bus.read(self.hl())?;
        self.sub_a_with(data);
        Ok(8)
    }

    fn sbc_a_with(&mut self, rhs: Word) {
        let carry: Word = if self.carry_flag() { 1 } else { 0 };
        let lhs = self.a();
        let rhs = rhs;
        let result = lhs.wrapping_sub(rhs).wrapping_sub(carry);
        self.zero_flag_mut().set_value(result == 0);
        self.negative_flag_mut().set();
        self.half_carry_flag_mut()
            .set_value(lhs & 0xF < rhs & 0xF + carry);
        self.carry_flag_mut().set_value(lhs < rhs + carry);
        *self.a_mut() = result as Word;
    }

    /// SBC A, B
    fn inst_0x98_sbc_a_b(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.sbc_a_with(self.b());
        Ok(4)
    }

    /// SBC A, C
    fn inst_0x99_sbc_a_c(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.sbc_a_with(self.c());
        Ok(4)
    }

    /// SBC A, D
    fn inst_0x9a_sbc_a_d(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.sbc_a_with(self.d());
        Ok(4)
    }

    /// SBC A, E
    fn inst_0x9b_sbc_a_e(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.sbc_a_with(self.e());
        Ok(4)
    }

    /// SBC A, H
    fn inst_0x9c_sbc_a_h(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.sbc_a_with(self.h());
        Ok(4)
    }

    /// SBC A, L
    fn inst_0x9d_sbc_a_l(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.sbc_a_with(self.l());
        Ok(4)
    }

    /// SBC A, A
    fn inst_0x9f_sbc_a_a(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.sbc_a_with(self.a());
        Ok(4)
    }

    /// SBC A, imm8
    fn inst_0xde_sbc_a_imm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.read_imm8bit(bus)?;
        self.sbc_a_with(data);
        Ok(8)
    }

    /// SBC A, (HL)
    fn inst_0x9e_sbc_a_mhl(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = bus.read(self.hl())?;
        self.sbc_a_with(data);
        Ok(8)
    }

    fn and_a_with(&mut self, rhs: Word) {
        let result = self.a() & rhs;
        self.zero_flag_mut().set_value(result == 0);
        self.negative_flag_mut().clear();
        self.half_carry_flag_mut().set();
        self.carry_flag_mut().clear();
        *self.a_mut() = result;
    }

    fn inst_0xa0_and_a_b(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.and_a_with(self.b());
        Ok(4)
    }

    fn inst_0xa1_and_a_c(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.and_a_with(self.c());
        Ok(4)
    }

    fn inst_0xa2_and_a_d(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.and_a_with(self.d());
        Ok(4)
    }

    fn inst_0xa3_and_a_e(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.and_a_with(self.e());
        Ok(4)
    }

    fn inst_0xa4_and_a_h(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.and_a_with(self.h());
        Ok(4)
    }

    fn inst_0xa5_and_a_l(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.and_a_with(self.l());
        Ok(4)
    }

    fn inst_0xa7_and_a_a(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.and_a_with(self.a());
        Ok(4)
    }

    fn inst_0xe6_and_a_imm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.read_imm8bit(bus)?;
        self.and_a_with(data);
        Ok(8)
    }

    fn inst_0xa6_and_a_mhl(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = bus.read(self.hl())?;
        self.and_a_with(data);
        Ok(8)
    }

    fn xor_a_with(&mut self, rhs: Word) {
        let result = self.a() ^ rhs;
        self.zero_flag_mut().set_value(result == 0);
        self.negative_flag_mut().clear();
        self.half_carry_flag_mut().clear();
        self.carry_flag_mut().clear();
        *self.a_mut() = result;
    }

    fn inst_0xa8_xor_a_b(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.xor_a_with(self.b());
        Ok(4)
    }

    fn inst_0xa9_xor_a_c(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.xor_a_with(self.c());
        Ok(4)
    }

    fn inst_0xaa_xor_a_d(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.xor_a_with(self.d());
        Ok(4)
    }

    fn inst_0xab_xor_a_e(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.xor_a_with(self.e());
        Ok(4)
    }

    fn inst_0xac_xor_a_h(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.xor_a_with(self.h());
        Ok(4)
    }

    fn inst_0xad_xor_a_l(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.xor_a_with(self.l());
        Ok(4)
    }

    fn inst_0xaf_xor_a_a(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.xor_a_with(self.a());
        Ok(4)
    }

    fn inst_0xee_xor_a_imm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.read_imm8bit(bus)?;
        self.xor_a_with(data);
        Ok(8)
    }

    fn inst_0xae_xor_a_mhl(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = bus.read(self.hl())?;
        self.xor_a_with(data);
        Ok(8)
    }

    fn or_a_with(&mut self, rhs: Word) {
        let result = self.a() | rhs;
        self.zero_flag_mut().set_value(result == 0);
        self.negative_flag_mut().clear();
        self.half_carry_flag_mut().clear();
        self.carry_flag_mut().clear();
        *self.a_mut() = result;
    }

    fn inst_0xb0_or_a_b(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.or_a_with(self.b());
        Ok(4)
    }

    fn inst_0xb1_or_a_c(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.or_a_with(self.c());
        Ok(4)
    }

    fn inst_0xb2_or_a_d(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.or_a_with(self.d());
        Ok(4)
    }

    fn inst_0xb3_or_a_e(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.or_a_with(self.e());
        Ok(4)
    }

    fn inst_0xb4_or_a_h(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.or_a_with(self.h());
        Ok(4)
    }

    fn inst_0xb5_or_a_l(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.or_a_with(self.l());
        Ok(4)
    }

    fn inst_0xb7_or_a_a(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.or_a_with(self.a());
        Ok(4)
    }

    fn inst_0xf6_or_a_imm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.read_imm8bit(bus)?;
        self.or_a_with(data);
        Ok(8)
    }

    fn inst_0xb6_or_a_mhl(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = bus.read(self.hl())?;
        self.or_a_with(data);
        Ok(8)
    }

    fn inst_0x2f_cpl_a(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.a_mut() ^= 0xFF;
        self.negative_flag_mut().set();
        self.half_carry_flag_mut().set();
        Ok(4)
    }

    fn inst_0x37_scf(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.carry_flag_mut().set();
        self.negative_flag_mut().clear();
        self.half_carry_flag_mut().clear();
        Ok(4)
    }

    fn inst_0x3f_ccf(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.carry_flag_mut().flip();
        self.negative_flag_mut().clear();
        self.half_carry_flag_mut().clear();
        Ok(4)
    }

    fn inst_0x27(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let a = self.a();
        let bcd = if self.negative_flag() {
            if self.carry_flag() {
                if self.half_carry_flag() {
                    a.wrapping_add(0x9A)
                } else {
                    a.wrapping_add(0xA0)
                }
            } else {
                if self.half_carry_flag() {
                    a.wrapping_add(0xFA)
                } else {
                    a
                }
            }
        } else {
            if self.carry_flag() || a > 0x99 {
                self.carry_flag_mut().set();
                if self.half_carry_flag() || a & 0x0F > 0x09 {
                    a.wrapping_add(0x66)
                } else {
                    a.wrapping_add(0x60)
                }
            } else {
                if self.half_carry_flag() || a & 0x0F > 0x09 {
                    a.wrapping_add(0x06)
                } else {
                    a
                }
            }
        };
        self.zero_flag_mut().set_value(bcd == 0);
        self.half_carry_flag_mut().clear();
        *self.a_mut() = bcd;
        Ok(4)
    }
}
