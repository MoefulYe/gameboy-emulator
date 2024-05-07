use self::{
    cb::{
        OPERAND_A, OPERAND_B, OPERAND_C, OPERAND_D, OPERAND_E, OPERAND_H, OPERAND_L, OPERAND_MHL,
    },
    ime::InterruptMasterEnableRegsiter,
    regs::Regs,
};
use crate::{
    cpu::cb::extended_inst_decode,
    dev::bus::{Bus, IO_LOW_BOUND},
    error::{EmulatorError, Result},
    types::{Addr, ClockCycle, DWord, OpCode, Word},
    utils::bits::BitMap,
};
use log::{error, info};

mod cb;
mod ime;
mod regs;
type Inst = fn(&mut CPU, &mut Bus) -> Result<ClockCycle>;

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

/// ref https://gbdev.io/pandocs/CPU_Instruction_Set.html
/// ref https://gbdev.io/gb-opcodes/optables/
/// ref https://archive.org/details/GameBoyProgManVer1.1/page/n93/mode/2up
/// Misc / control instructions
impl CPU {
    fn decode_inst(opcode: OpCode) -> Inst {
        const INSTS: &[Inst; 256] = &[
            CPU::inst_0x00_nop,
            CPU::inst_0x01_ld_bc_imm16,
            CPU::inst_0x02_ld_mbc_a,
            CPU::inst_0x03_inc_bc,
            CPU::inst_0x04_inc_b,
            CPU::inst_0x05_dec_b,
            CPU::inst_0x06_ld_b_imm8,
            CPU::inst_0x07_rlca,
            CPU::inst_0x08_ld_mimm16_sp,
            CPU::inst_0x09_add_hl_bc,
            CPU::inst_0x0a_ld_a_mbc,
            CPU::inst_0x0b_dec_bc,
            CPU::inst_0x0c_inc_c,
            CPU::inst_0x0d_dec_c,
            CPU::inst_0x0e_ld_c_imm8,
            CPU::inst_0x0f_rrca,
            CPU::inst_0x10_stop,
            CPU::inst_0x11_ld_de_imm16,
            CPU::inst_0x12_ld_mde_a,
            CPU::inst_0x13_inc_de,
            CPU::inst_0x14_inc_d,
            CPU::inst_0x15_dec_d,
            CPU::inst_0x16_ld_d_imm8,
            CPU::inst_0x17_rla,
            CPU::inst_0x18_jr_imm8,
            CPU::inst_0x19_add_hl_de,
            CPU::inst_0x1a_ld_a_mde,
            CPU::inst_0x1b_dec_de,
            CPU::inst_0x1c_inc_e,
            CPU::inst_0x1d_dec_e,
            CPU::inst_0x1e_ld_e_imm8,
            CPU::inst_0x1f_rra,
            CPU::inst_0x20_jr_nz_imm8,
            CPU::inst_0x21_ld_hl_imm16,
            CPU::inst_0x22_ldi_mhl_a,
            CPU::inst_0x23_inc_hl,
            CPU::inst_0x24_inc_h,
            CPU::inst_0x25_dec_h,
            CPU::inst_0x26_ld_h_imm8,
            CPU::inst_0x27_daa,
            CPU::inst_0x28_jr_z_imm8,
            CPU::inst_0x29_add_hl_hl,
            CPU::inst_0x2a_ldi_a_mhl,
            CPU::inst_0x2b_dec_hl,
            CPU::inst_0x2c_inc_l,
            CPU::inst_0x2d_dec_l,
            CPU::inst_0x2e_ld_l_imm8,
            CPU::inst_0x2f_cpl_a,
            CPU::inst_0x30_jr_nc_imm8,
            CPU::inst_0x31_ld_sp_imm16,
            CPU::inst_0x32_ldd_mhl_a,
            CPU::inst_0x33_inc_sp,
            CPU::inst_0x34_inc_mhl,
            CPU::inst_0x35_dec_mhl,
            CPU::inst_0x36_ld_mdl_imm8,
            CPU::inst_0x37_scf,
            CPU::inst_0x38_jr_c_imm8,
            CPU::inst_0x39_add_hl_sp,
            CPU::inst_0x3a_ldd_a_mhl,
            CPU::inst_0x3b_dec_sp,
            CPU::inst_0x3c_inc_a,
            CPU::inst_0x3d_dec_a,
            CPU::inst_0x3e_ld_a_imm8,
            CPU::inst_0x3f_ccf,
            CPU::inst_0x40_ld_b_b,
            CPU::inst_0x41_ld_b_c,
            CPU::inst_0x42_ld_b_d,
            CPU::inst_0x43_ld_b_e,
            CPU::inst_0x44_ld_b_h,
            CPU::inst_0x45_ld_b_l,
            CPU::inst_0x46_ld_b_mhl,
            CPU::inst_0x47_ld_b_a,
            CPU::inst_0x48_ld_c_b,
            CPU::inst_0x49_ld_c_c,
            CPU::inst_0x4a_ld_c_d,
            CPU::inst_0x4b_ld_c_e,
            CPU::inst_0x4c_ld_c_h,
            CPU::inst_0x4d_ld_c_l,
            CPU::inst_0x4e_ld_c_mhl,
            CPU::inst_0x4f_ld_c_a,
            CPU::inst_0x50_ld_d_b,
            CPU::inst_0x51_ld_d_c,
            CPU::inst_0x52_ld_d_d,
            CPU::inst_0x53_ld_d_e,
            CPU::inst_0x54_ld_d_h,
            CPU::inst_0x55_ld_d_l,
            CPU::inst_0x56_ld_d_mhl,
            CPU::inst_0x57_ld_d_a,
            CPU::inst_0x58_ld_e_b,
            CPU::inst_0x59_ld_e_c,
            CPU::inst_0x5a_ld_e_d,
            CPU::inst_0x5b_ld_e_e,
            CPU::inst_0x5c_ld_e_h,
            CPU::inst_0x5d_ld_e_l,
            CPU::inst_0x5e_ld_e_mhl,
            CPU::inst_0x5f_ld_e_a,
            CPU::inst_0x60_ld_h_b,
            CPU::inst_0x61_ld_h_c,
            CPU::inst_0x62_ld_h_d,
            CPU::inst_0x63_ld_h_e,
            CPU::inst_0x64_ld_h_h,
            CPU::inst_0x65_ld_h_l,
            CPU::inst_0x66_ld_h_mhl,
            CPU::inst_0x67_ld_h_a,
            CPU::inst_0x68_ld_l_b,
            CPU::inst_0x69_ld_l_c,
            CPU::inst_0x6a_ld_l_d,
            CPU::inst_0x6b_ld_l_e,
            CPU::inst_0x6c_ld_l_h,
            CPU::inst_0x6d_ld_l_l,
            CPU::inst_0x6e_ld_l_mhl,
            CPU::inst_0x6f_ld_l_a,
            CPU::inst_0x70_ld_mhl_b,
            CPU::inst_0x71_ld_mhl_c,
            CPU::inst_0x72_ld_mhl_d,
            CPU::inst_0x73_ld_mhl_e,
            CPU::inst_0x74_ld_mhl_h,
            CPU::inst_0x75_ld_mhl_l,
            CPU::inst_0x76_halt,
            CPU::inst_0x77_ld_mhl_a,
            CPU::inst_0x78_ld_a_b,
            CPU::inst_0x79_ld_a_c,
            CPU::inst_0x7a_ld_a_d,
            CPU::inst_0x7b_ld_a_e,
            CPU::inst_0x7c_ld_a_h,
            CPU::inst_0x7d_ld_a_l,
            CPU::inst_0x7e_ld_a_mhl,
            CPU::inst_0x7f_ld_a_a,
            CPU::inst_0x80_add_a_b,
            CPU::inst_0x81_add_a_c,
            CPU::inst_0x82_add_a_d,
            CPU::inst_0x83_add_a_e,
            CPU::inst_0x84_add_a_h,
            CPU::inst_0x85_add_a_l,
            CPU::inst_0x86_add_a_mhl,
            CPU::inst_0x87_add_a_a,
            CPU::inst_0x88_adc_a_b,
            CPU::inst_0x89_adc_a_c,
            CPU::inst_0x8a_adc_a_d,
            CPU::inst_0x8b_adc_a_e,
            CPU::inst_0x8c_adc_a_h,
            CPU::inst_0x8d_adc_a_l,
            CPU::inst_0x8e_adc_a_mhl,
            CPU::inst_0x8f_adc_a_a,
            CPU::inst_0x90_sub_a_b,
            CPU::inst_0x91_sub_a_c,
            CPU::inst_0x92_sub_a_d,
            CPU::inst_0x93_sub_a_e,
            CPU::inst_0x94_sub_a_h,
            CPU::inst_0x95_sub_a_l,
            CPU::inst_0x96_sub_a_mhl,
            CPU::inst_0x97_sub_a_a,
            CPU::inst_0x98_sbc_a_b,
            CPU::inst_0x99_sbc_a_c,
            CPU::inst_0x9a_sbc_a_d,
            CPU::inst_0x9b_sbc_a_e,
            CPU::inst_0x9c_sbc_a_h,
            CPU::inst_0x9d_sbc_a_l,
            CPU::inst_0x9e_sbc_a_mhl,
            CPU::inst_0x9f_sbc_a_a,
            CPU::inst_0xa0_and_a_b,
            CPU::inst_0xa1_and_a_c,
            CPU::inst_0xa2_and_a_d,
            CPU::inst_0xa3_and_a_e,
            CPU::inst_0xa4_and_a_h,
            CPU::inst_0xa5_and_a_l,
            CPU::inst_0xa6_and_a_mhl,
            CPU::inst_0xa7_and_a_a,
            CPU::inst_0xa8_xor_a_b,
            CPU::inst_0xa9_xor_a_c,
            CPU::inst_0xaa_xor_a_d,
            CPU::inst_0xab_xor_a_e,
            CPU::inst_0xac_xor_a_h,
            CPU::inst_0xad_xor_a_l,
            CPU::inst_0xae_xor_a_mhl,
            CPU::inst_0xaf_xor_a_a,
            CPU::inst_0xb0_or_a_b,
            CPU::inst_0xb1_or_a_c,
            CPU::inst_0xb2_or_a_d,
            CPU::inst_0xb3_or_a_e,
            CPU::inst_0xb4_or_a_h,
            CPU::inst_0xb5_or_a_l,
            CPU::inst_0xb6_or_a_mhl,
            CPU::inst_0xb7_or_a_a,
            CPU::inst_0xb8_cp_b,
            CPU::inst_0xb9_cp_c,
            CPU::inst_0xba_cp_d,
            CPU::inst_0xbb_cp_e,
            CPU::inst_0xbc_cp_h,
            CPU::inst_0xbd_cp_l,
            CPU::inst_0xbe_cp_mhl,
            CPU::inst_0xbf_cp_a,
            CPU::inst_0xc0_ret_nz,
            CPU::inst_0xc1_pop_bc,
            CPU::inst_0xc2_jp_nz_imm16,
            CPU::inst_0xc3_jp_imm16,
            CPU::inst_0xc4_call_nz_imm16,
            CPU::inst_0xc5_push_bc,
            CPU::inst_0xc6_add_a_imm8,
            CPU::inst_0xc7_rst_0x0000,
            CPU::inst_0xc8_ret_z,
            CPU::inst_0xc9_ret,
            CPU::inst_0xca_jp_z_imm16,
            CPU::inst_0xcb_prefix_cb,
            CPU::inst_0xcc_call_z_imm16,
            CPU::inst_0xcd_call_imm16,
            CPU::inst_0xce_add_a_imm8,
            CPU::inst_0xcf_rst_0x0008,
            CPU::inst_0xd0_ret_nc,
            CPU::inst_0xd1_pop_de,
            CPU::inst_0xd2_jp_nc_imm16,
            CPU::inst_illegal,
            CPU::inst_0xd4_call_nc_imm16,
            CPU::inst_0xd5_push_de,
            CPU::inst_0xd6_sub_a_imm8,
            CPU::inst_0xd7_rst_0x0010,
            CPU::inst_0xd8_ret_c,
            CPU::inst_0xd9_reti,
            CPU::inst_0xda_jp_c_imm16,
            CPU::inst_illegal,
            CPU::inst_0xdc_call_c_imm16,
            CPU::inst_illegal,
            CPU::inst_0xde_sbc_a_imm8,
            CPU::inst_0xdf_rst_0x0018,
            CPU::inst_0xe0_ldh_mimm8_a,
            CPU::inst_0xe1_pop_hl,
            CPU::inst_0xe2_ldh_mc_a,
            CPU::inst_illegal,
            CPU::inst_illegal,
            CPU::inst_0xe5_push_hl,
            CPU::inst_0xe6_and_a_imm8,
            CPU::inst_0xe7_rst_0x0020,
            CPU::inst_0xe8_add_sp_imm8,
            CPU::inst_0xe9_jp_hl,
            CPU::inst_0xea_ld_mimm16_a,
            CPU::inst_illegal,
            CPU::inst_illegal,
            CPU::inst_illegal,
            CPU::inst_0xee_xor_a_imm8,
            CPU::inst_0xef_rst_0x0028,
            CPU::inst_0xf0_ldh_a_mimm8,
            CPU::inst_0xf1_pop_af,
            CPU::inst_0xf2_ldh_a_mc,
            CPU::inst_0xf3_di,
            CPU::inst_illegal,
            CPU::inst_0xf5_push_af,
            CPU::inst_0xf6_or_a_imm8,
            CPU::inst_0xf7_rst_0x0030,
            CPU::inst_0xf8_ld_hl_sp_imm8,
            CPU::inst_0xf9_ld_sp_hl,
            CPU::inst_0xfa_ld_a_mimm16,
            CPU::inst_0xfb_ei,
            CPU::inst_illegal,
            CPU::inst_illegal,
            CPU::inst_0xfe_cp_imm8,
            CPU::inst_0xff_rst_0x0038,
        ];
        *unsafe { INSTS.get_unchecked(opcode as usize) }
    }

    fn mnemonic(opcode: OpCode) -> &'static str {
        const MNEMONICS: &[&'static str; 256] = &[
            "0x00 NOP",
            "0x01 LD BC IMM16",
            "0x02 LD (BC) A",
            "0x03 INC BC",
            "0x04 INC B",
            "0x05 DEC B",
            "0x06 LD B IMM8",
            "0x07 RLCA",
            "0x08 LD (IMM16) SP",
            "0x09 ADD HL BC",
            "0x0A LD A (BC)",
            "0x0B DEC BC",
            "0x0C INC C",
            "0x0D DEC C",
            "0x0E LD C IMM8",
            "0x0F RRCA",
            "0x10 STOP",
            "0x11 LD DE IMM16",
            "0x12 LD (DE) A",
            "0x13 INC DE",
            "0x14 INC D",
            "0x15 DEC D",
            "0x16 LD D IMM8",
            "0x17 RLA",
            "0x18 JR IMM8",
            "0x19 ADD HL DE",
            "0x1A LD A (DE)",
            "0x1B DEC DE",
            "0x1C INC E",
            "0x1D DEC E",
            "0x1E LD E IMM8",
            "0x1F RRA",
            "0x20 JR NZ IMM8",
            "0x21 LD HL IMM16",
            "0x22 LD (HL+) A",
            "0x23 INC HL",
            "0x24 INC H",
            "0x25 DEC H",
            "0x26 LD H IMM8",
            "0x27 DAA",
            "0x28 JR Z IMM8",
            "0x29 ADD HL HL",
            "0x2A LD A (HL+)",
            "0x2B DEC HL",
            "0x2C INC L",
            "0x2D DEC L",
            "0x2E LD L IMM8",
            "0x2F CPL A",
            "0x30 JR NC IMM8",
            "0x31 LD SP IMM16",
            "0x32 LD (HL-) A",
            "0x33 INC SP",
            "0x34 INC (HL)",
            "0x35 DEC (HL)",
            "0x36 LD (HL) IMM8",
            "0x37 SCF",
            "0x38 JR C IMM8",
            "0x39 ADD HL SP",
            "0x3A LD A (HL-)",
            "0x3B DEC SP",
            "0x3C INC A",
            "0x3D DEC A",
            "0x3E LD A IMM8",
            "0x3F CCF",
            "0x40 LD B B",
            "0x41 LD B C",
            "0x42 LD B D",
            "0x43 LD B E",
            "0x44 LD B H",
            "0x45 LD B L",
            "0x46 LD B (HL)",
            "0x47 LD B A",
            "0x48 LD C B",
            "0x49 LD C C",
            "0x4A LD C D",
            "0x4B LD C E",
            "0x4C LD C H",
            "0x4D LD C L",
            "0x4E LD C (HL)",
            "0x4F LD C A",
            "0x50 LD D B",
            "0x51 LD D C",
            "0x52 LD D D",
            "0x53 LD D E",
            "0x54 LD D H",
            "0x55 LD D L",
            "0x56 LD D (HL)",
            "0x57 LD D A",
            "0x58 LD E B",
            "0x59 LD E C",
            "0x5A LD E D",
            "0x5B LD E E",
            "0x5C LD E H",
            "0x5D LD E L",
            "0x5E LD E (HL)",
            "0x5F LD E A",
            "0x60 LD H B",
            "0x61 LD H C",
            "0x62 LD H D",
            "0x63 LD H E",
            "0x64 LD H H",
            "0x65 LD H L",
            "0x66 LD H (HL)",
            "0x67 LD H A",
            "0x68 LD L B",
            "0x69 LD L C",
            "0x6A LD L D",
            "0x6B LD L E",
            "0x6C LD L H",
            "0x6D LD L L",
            "0x6E LD L (HL)",
            "0x6F LD L A",
            "0x70 LD (HL) B",
            "0x71 LD (HL) C",
            "0x72 LD (HL) D",
            "0x73 LD (HL) E",
            "0x74 LD (HL) H",
            "0x75 LD (HL) L",
            "0x76 HALT",
            "0x77 LD (HL) A",
            "0x78 LD A B",
            "0x79 LD A C",
            "0x7A LD A D",
            "0x7B LD A E",
            "0x7C LD A H",
            "0x7D LD A L",
            "0x7E LD A (HL)",
            "0x7F LD A A",
            "0x80 ADD A B",
            "0x81 ADD A C",
            "0x82 ADD A D",
            "0x83 ADD A E",
            "0x84 ADD A H",
            "0x85 ADD A L",
            "0x86 ADD A (HL)",
            "0x87 ADD A A",
            "0x88 ADC A B",
            "0x89 ADC A C",
            "0x8A ADC A D",
            "0x8B ADC A E",
            "0x8C ADC A H",
            "0x8D ADC A L",
            "0x8E ADC A (HL)",
            "0x8F ADC A A",
            "0x90 SUB A B",
            "0x91 SUB A C",
            "0x92 SUB A D",
            "0x93 SUB A E",
            "0x94 SUB A H",
            "0x95 SUB A L",
            "0x96 SUB A (HL)",
            "0x97 SUB A A",
            "0x98 SBC A B",
            "0x99 SBC A C",
            "0x9A SBC A D",
            "0x9B SBC A E",
            "0x9C SBC A H",
            "0x9D SBC A L",
            "0x9E SBC A (HL)",
            "0x9F SBC A A",
            "0xA0 AND A B",
            "0xA1 AND A C",
            "0xA2 AND A D",
            "0xA3 AND A E",
            "0xA4 AND A H",
            "0xA5 AND A L",
            "0xA6 AND A (HL)",
            "0xA7 AND A A",
            "0xA8 XOR A B",
            "0xA9 XOR A C",
            "0xAA XOR A D",
            "0xAB XOR A E",
            "0xAC XOR A H",
            "0xAD XOR A L",
            "0xAE XOR A (HL)",
            "0xAF XOR A A",
            "0xB0 OR A B",
            "0xB1 OR A C",
            "0xB2 OR A D",
            "0xB3 OR A E",
            "0xB4 OR A H",
            "0xB5 OR A L",
            "0xB6 OR A (HL)",
            "0xB7 OR A A",
            "0xB8 CP B",
            "0xB9 CP C",
            "0xBA CP D",
            "0xBB CP E",
            "0xBC CP H",
            "0xBD CP L",
            "0xBE CP (HL)",
            "0xBF CP A",
            "0xC0 RET NZ",
            "0xC1 POP BC",
            "0xC2 JP NZ IMM16",
            "0xC3 JP IMM16",
            "0xC4 CALL NZ IMM16",
            "0xC5 PUSH BC",
            "0xC6 ADD A IMM8",
            "0xC7 RST 0x0000",
            "0xC8 RET Z",
            "0xC9 RET",
            "0xCA JP Z IMM16",
            "0xCB PREFIX CB",
            "0xCC CALL Z IMM16",
            "0xCD CALL IMM16",
            "0xCE ADC A IMM8",
            "0xCF RST 0x0008",
            "0xD0 RET NC",
            "0xD1 POP DE",
            "0xD2 JP NC IMM16",
            "0xD3 ILLEGAL",
            "0xD4 CALL NC IMM16",
            "0xD5 PUSH DE",
            "0xD6 SUB A IMM8",
            "0xD7 RST 0x0010",
            "0xD8 RET C",
            "0xD9 RETI",
            "0xDA JP C IMM16",
            "0xDB ILLEGAL",
            "0xDC CALL C IMM16",
            "0xDD ILLEGAL",
            "0xDE SBC A IMM8",
            "0xDF RST 0x0018",
            "0xE0 LDH (0xFF00+IMM8) A",
            "0xE1 POP HL",
            "0xE2 LD (0xFF00+C) A",
            "0xE3 ILLEGAL",
            "0xE4 ILLEGAL",
            "0xE5 PUSH HL",
            "0xE6 AND A IMM8",
            "0xE7 RST 0x0020",
            "0xE8 ADD SP IMM8",
            "0xE9 JP HL",
            "0xEA LD (IMM16) A",
            "0xEB ILLEGAL",
            "0xEC ILLEGAL",
            "0xED ILLEGAL",
            "0xEE XOR A IMM8",
            "0xEF RST 0x0028",
            "0xF0 LDH A (0xFF00+IMM8)",
            "0xF1 POP AF",
            "0xF2 A (0xFF00 + C)",
            "0xF3 DI",
            "0xF4 ILLEGAL",
            "0xF5 PUSH AF",
            "0xF6 OR A IMM8",
            "0xF7 RST 0x0030",
            "0xF8 LD HL SP+IMM8",
            "0xF9 LD SP HL",
            "0xFA LD A (IMM16)",
            "0xFB EI",
            "0xFC ILLEGAL",
            "0xFD ILLEGAL",
            "0xFE CP IMM8",
            "0xFF RST 0x0038",
        ];
        *unsafe { MNEMONICS.get_unchecked(opcode as usize) }
    }

    fn inst_0x00_nop(_: &mut CPU, _: &mut Bus) -> Result<ClockCycle> {
        Ok(4)
    }

    fn inst_illegal(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let addr = self.pc() - 1;
        let opcode = bus.read(addr)?;
        error!("illegal opcode: 0x{opcode:02X} at address: 0x{addr:04X}");
        Err(EmulatorError::IllegalInstruction)
    }

    fn inst_0x76_halt(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.halted = true;
        Ok(4)
    }

    fn inst_0x10_stop(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        info!("stop instruction at address: 0x{:04X}", self.pc() - 1);
        Err(EmulatorError::StopInstruction)
    }

    fn inst_0xf3_di(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.ime.disable();
        Ok(4)
    }

    fn inst_0xfb_ei(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.ime.enable();
        Ok(4)
    }
}

/// LD between 8bit registers instructions
/// LD dest, src
impl CPU {
    fn inst_0x40_ld_b_b(_: &mut CPU, _: &mut Bus) -> Result<ClockCycle> {
        Ok(4)
    }

    fn inst_0x41_ld_b_c(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.b_mut() = self.c();
        Ok(4)
    }

    fn inst_0x42_ld_b_d(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.b_mut() = self.d();
        Ok(4)
    }

    fn inst_0x43_ld_b_e(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.b_mut() = self.e();
        Ok(4)
    }

    fn inst_0x44_ld_b_h(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.b_mut() = self.h();
        Ok(4)
    }

    fn inst_0x45_ld_b_l(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.b_mut() = self.l();
        Ok(4)
    }

    fn inst_0x47_ld_b_a(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.b_mut() = self.a();
        Ok(4)
    }

    fn inst_0x48_ld_c_b(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.c_mut() = self.b();
        Ok(4)
    }

    fn inst_0x49_ld_c_c(_: &mut CPU, _: &mut Bus) -> Result<ClockCycle> {
        Ok(4)
    }

    fn inst_0x4a_ld_c_d(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.c_mut() = self.d();
        Ok(4)
    }

    fn inst_0x4b_ld_c_e(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.c_mut() = self.e();
        Ok(4)
    }

    fn inst_0x4c_ld_c_h(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.c_mut() = self.h();
        Ok(4)
    }

    fn inst_0x4d_ld_c_l(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.c_mut() = self.l();
        Ok(4)
    }

    fn inst_0x4f_ld_c_a(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.c_mut() = self.a();
        Ok(4)
    }

    fn inst_0x50_ld_d_b(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.d_mut() = self.b();
        Ok(4)
    }

    fn inst_0x51_ld_d_c(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.d_mut() = self.c();
        Ok(4)
    }

    fn inst_0x52_ld_d_d(_: &mut CPU, _: &mut Bus) -> Result<ClockCycle> {
        Ok(4)
    }

    fn inst_0x53_ld_d_e(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.d_mut() = self.e();
        Ok(4)
    }

    fn inst_0x54_ld_d_h(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.d_mut() = self.h();
        Ok(4)
    }

    fn inst_0x55_ld_d_l(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.d_mut() = self.l();
        Ok(4)
    }

    fn inst_0x57_ld_d_a(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.d_mut() = self.a();
        Ok(4)
    }

    fn inst_0x58_ld_e_b(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.e_mut() = self.b();
        Ok(4)
    }

    fn inst_0x59_ld_e_c(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.e_mut() = self.c();
        Ok(4)
    }

    fn inst_0x5a_ld_e_d(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.e_mut() = self.d();
        Ok(4)
    }

    fn inst_0x5b_ld_e_e(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        Ok(4)
    }

    fn inst_0x5c_ld_e_h(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.e_mut() = self.h();
        Ok(4)
    }

    fn inst_0x5d_ld_e_l(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.e_mut() = self.l();
        Ok(4)
    }

    fn inst_0x5f_ld_e_a(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.e_mut() = self.a();
        Ok(4)
    }

    fn inst_0x60_ld_h_b(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.h_mut() = self.b();
        Ok(4)
    }

    fn inst_0x61_ld_h_c(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.h_mut() = self.c();
        Ok(4)
    }

    fn inst_0x62_ld_h_d(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.h_mut() = self.d();
        Ok(4)
    }

    fn inst_0x63_ld_h_e(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.h_mut() = self.e();
        Ok(4)
    }

    fn inst_0x64_ld_h_h(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        Ok(4)
    }

    fn inst_0x65_ld_h_l(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.h_mut() = self.l();
        Ok(4)
    }

    fn inst_0x67_ld_h_a(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.h_mut() = self.a();
        Ok(4)
    }

    fn inst_0x68_ld_l_b(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.l_mut() = self.b();
        Ok(4)
    }

    fn inst_0x69_ld_l_c(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.l_mut() = self.c();
        Ok(4)
    }

    fn inst_0x6a_ld_l_d(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.l_mut() = self.d();
        Ok(4)
    }

    fn inst_0x6b_ld_l_e(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.l_mut() = self.e();
        Ok(4)
    }

    fn inst_0x6c_ld_l_h(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.l_mut() = self.h();
        Ok(4)
    }

    fn inst_0x6d_ld_l_l(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        Ok(4)
    }

    fn inst_0x6f_ld_l_a(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.l_mut() = self.a();
        Ok(4)
    }

    fn inst_0x78_ld_a_b(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.a_mut() = self.b();
        Ok(4)
    }

    fn inst_0x79_ld_a_c(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.a_mut() = self.c();
        Ok(4)
    }

    fn inst_0x7a_ld_a_d(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.a_mut() = self.d();
        Ok(4)
    }

    fn inst_0x7b_ld_a_e(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.a_mut() = self.e();
        Ok(4)
    }

    fn inst_0x7c_ld_a_h(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.a_mut() = self.h();
        Ok(4)
    }

    fn inst_0x7d_ld_a_l(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.a_mut() = self.l();
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
        let bc = self.bc();
        let data = bus.read(bc)?;
        *self.a_mut() = data;
        Ok(8)
    }

    fn inst_0x1a_ld_a_mde(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let de = self.de();
        let data = bus.read(de)?;
        *self.a_mut() = data;
        Ok(8)
    }

    fn inst_0x46_ld_b_mhl(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let hl = self.hl();
        let data = bus.read(hl)?;
        *self.b_mut() = data;
        Ok(8)
    }

    fn inst_0x4e_ld_c_mhl(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let hl = self.hl();
        let data = bus.read(hl)?;
        *self.c_mut() = data;
        Ok(8)
    }

    fn inst_0x56_ld_d_mhl(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let hl = self.hl();
        let data = bus.read(hl)?;
        *self.d_mut() = data;
        Ok(8)
    }

    fn inst_0x5e_ld_e_mhl(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let hl = self.hl();
        let data = bus.read(hl)?;
        *self.e_mut() = data;
        Ok(8)
    }

    fn inst_0x66_ld_h_mhl(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let hl = self.hl();
        let data = bus.read(hl)?;
        *self.h_mut() = data;
        Ok(8)
    }

    fn inst_0x6e_ld_l_mhl(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let hl = self.hl();
        let data = bus.read(hl)?;
        *self.l_mut() = data;
        Ok(8)
    }

    fn inst_0x7e_ld_a_mhl(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let hl = self.hl();
        let data = bus.read(hl)?;
        *self.a_mut() = data;
        Ok(8)
    }
}

/// LD from 8bit register to memory instructions
/// LD (16 bits register pointers to memory), src
impl CPU {
    fn inst_0x02_ld_mbc_a(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let bc = self.bc();
        let data = self.a();
        bus.write(bc, data)?;
        Ok(8)
    }

    fn inst_0x12_ld_mde_a(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let de = self.de();
        let data = self.a();
        bus.write(de, data)?;
        Ok(8)
    }

    fn inst_0x70_ld_mhl_b(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let hl = self.hl();
        let data = self.b();
        bus.write(hl, data)?;
        Ok(8)
    }

    fn inst_0x71_ld_mhl_c(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let hl = self.hl();
        let data = self.c();
        bus.write(hl, data)?;
        Ok(8)
    }

    fn inst_0x72_ld_mhl_d(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let hl = self.hl();
        let data = self.d();
        bus.write(hl, data)?;
        Ok(8)
    }

    fn inst_0x73_ld_mhl_e(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let hl = self.hl();
        let data = self.e();
        bus.write(hl, data)?;
        Ok(8)
    }

    fn inst_0x74_ld_mhl_h(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let hl = self.hl();
        let data = self.h();
        bus.write(hl, data)?;
        Ok(8)
    }

    fn inst_0x75_ld_mhl_l(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let hl = self.hl();
        let data = self.l();
        bus.write(hl, data)?;
        Ok(8)
    }

    fn inst_0x77_ld_mhl_a(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let hl = self.hl();
        let data = self.a();
        bus.write(hl, data)?;
        Ok(8)
    }
}

/// special LD between 8bit registers and memory instructions
impl CPU {
    /// LD (HL+), A
    fn inst_0x22_ldi_mhl_a(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let hl = self.hl();
        let data = self.a();
        bus.write(hl, data)?;
        *self.hl_mut() = self.hl_mut().wrapping_add(1);
        Ok(8)
    }

    /// LD A, (HL+)
    fn inst_0x2a_ldi_a_mhl(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let hl = self.hl();
        let data = bus.read(hl)?;
        *self.a_mut() = data;
        *self.hl_mut() = self.hl_mut().wrapping_add(1);
        Ok(8)
    }

    /// LD (HL-), A
    fn inst_0x32_ldd_mhl_a(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let hl = self.hl();
        let data = self.a();
        bus.write(hl, data)?;
        *self.hl_mut() = self.hl_mut().wrapping_sub(1);
        Ok(8)
    }

    /// LD A, (HL-)
    fn inst_0x3a_ldd_a_mhl(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let hl = self.hl();
        let data = bus.read(hl)?;
        *self.a_mut() = data;
        *self.hl_mut() = self.hl_mut().wrapping_sub(1);
        Ok(8)
    }

    /// LDH (C), A
    fn inst_0xe2_ldh_mc_a(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let addr = IO_LOW_BOUND + self.c() as Addr;
        let data = self.a();
        bus.write(addr, data)?;
        Ok(8)
    }

    /// LDH A, (C)
    fn inst_0xf2_ldh_a_mc(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let addr = IO_LOW_BOUND + self.c() as Addr;
        let data = bus.read(addr)?;
        *self.a_mut() = data;
        Ok(8)
    }
}

/// LD from immediate 8bit data to 8bit register instructions
impl CPU {
    #[inline]
    fn read_word(&mut self, bus: &mut Bus) -> Result<Word> {
        let pc = self.pc();
        let data = bus.read(pc)?;
        self.pc_inc();
        Ok(data)
    }

    fn inst_0x06_ld_b_imm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.read_word(bus)?;
        *self.b_mut() = data;
        Ok(8)
    }

    fn inst_0x0e_ld_c_imm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.read_word(bus)?;
        *self.c_mut() = data;
        Ok(8)
    }

    fn inst_0x16_ld_d_imm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.read_word(bus)?;
        *self.d_mut() = data;
        Ok(8)
    }

    fn inst_0x1e_ld_e_imm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.read_word(bus)?;
        *self.e_mut() = data;
        Ok(8)
    }

    fn inst_0x26_ld_h_imm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.read_word(bus)?;
        *self.h_mut() = data;
        Ok(8)
    }

    fn inst_0x2e_ld_l_imm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.read_word(bus)?;
        *self.l_mut() = data;
        Ok(8)
    }

    fn inst_0x3e_ld_a_imm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.read_word(bus)?;
        *self.a_mut() = data;
        Ok(8)
    }
}

/// LD (16 bits register pointers to memory), immediate 8bit data
impl CPU {
    fn inst_0x36_ld_mdl_imm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let word = self.read_word(bus)?;
        let hl = self.hl();
        bus.write(hl, word)?;
        Ok(12)
    }
}

/// LDH between (0xFF00 + immediate 8bit data) and A instructions
impl CPU {
    fn inst_0xe0_ldh_mimm8_a(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let imme8 = self.read_word(bus)?;
        let addr = IO_LOW_BOUND + imme8 as Addr;
        let a = self.a();
        bus.write(addr, a)?;
        Ok(12)
    }

    fn inst_0xf0_ldh_a_mimm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let imme8 = self.read_word(bus)?;
        let addr = IO_LOW_BOUND + imme8 as Addr;
        let data = bus.read(addr)?;
        *self.a_mut() = data;
        Ok(12)
    }
}

/// LD from 16bit immediate data to 16bit register instructions
impl CPU {
    #[inline]
    fn read_dword(&mut self, bus: &mut Bus) -> Result<DWord> {
        let pc = self.pc();
        let low = bus.read(pc)?;
        let high = bus.read(pc + 1)?;
        self.pc_inc_by(2);
        let ret = (high as DWord) << 8 | low as DWord;
        Ok(ret)
    }

    fn inst_0x01_ld_bc_imm16(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.read_dword(bus)?;
        *self.bc_mut() = data;
        Ok(12)
    }

    fn inst_0x11_ld_de_imm16(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.read_dword(bus)?;
        *self.de_mut() = data;
        Ok(12)
    }

    fn inst_0x21_ld_hl_imm16(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.read_dword(bus)?;
        *self.hl_mut() = data;
        Ok(12)
    }

    fn inst_0x31_ld_sp_imm16(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.read_dword(bus)?;
        *self.sp_mut() = data;
        Ok(12)
    }
}

/// LD from SP to (16 bits register pointers to memory)
impl CPU {
    fn inst_0x08_ld_mimm16_sp(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let addr = self.read_dword(bus)?;
        let sp = self.sp();
        let low = (sp & 0xFF) as Word;
        let high = (sp >> 8) as Word;
        bus.write(addr, low)?;
        bus.write(addr + 1, high)?;
        Ok(20)
    }
}

/// LD from HL to SP instructions
impl CPU {
    fn inst_0xf9_ld_sp_hl(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        *self.sp_mut() = self.hl();
        Ok(8)
    }
}

/// LD between 16bit immediate pointers to memory and A
impl CPU {
    fn inst_0xea_ld_mimm16_a(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let addr = self.read_dword(bus)?;
        let a = self.a();
        bus.write(addr, a)?;
        Ok(16)
    }

    fn inst_0xfa_ld_a_mimm16(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let addr = self.read_dword(bus)?;
        let data = bus.read(addr)?;
        *self.a_mut() = data;
        Ok(16)
    }
}

impl CPU {
    /// LD HL, SP + imme8
    fn inst_0xf8_ld_hl_sp_imm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let imme8 = self.read_word(bus)? as i8 as i16;
        self.zero_flag_mut().clear();
        self.negative_flag_mut().clear();
        let sp = self.sp() as i16;
        let result = sp.wrapping_add(imme8);
        let check = sp ^ imme8 ^ result;
        self.half_carry_flag_mut().set_value(check & 0x10 != 0);
        self.carry_flag_mut().set_value(check & 0x100 != 0);
        Ok(12)
    }
}

/// CP
impl CPU {
    fn cp_a_with(&mut self, val: Word) {
        let a = self.a();
        self.zero_flag_mut().set_value(a == val);
        self.negative_flag_mut().set();
        self.half_carry_flag_mut()
            .set_value((a & 0xF) < (val & 0xF));
        self.carry_flag_mut().set_value(a < val);
    }

    fn inst_0xb8_cp_b(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.cp_a_with(self.b());
        Ok(4)
    }

    fn inst_0xb9_cp_c(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.cp_a_with(self.c());
        Ok(4)
    }

    fn inst_0xba_cp_d(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.cp_a_with(self.d());
        Ok(4)
    }

    fn inst_0xbb_cp_e(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.cp_a_with(self.e());
        Ok(4)
    }

    fn inst_0xbc_cp_h(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.cp_a_with(self.h());
        Ok(4)
    }

    fn inst_0xbd_cp_l(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.cp_a_with(self.l());
        Ok(4)
    }

    fn inst_0xbf_cp_a(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.cp_a_with(self.a());
        Ok(4)
    }

    /// CP (HL)
    fn inst_0xbe_cp_mhl(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let hl = self.hl();
        let data = bus.read(hl)?;
        self.cp_a_with(data);
        Ok(8)
    }

    /// CP imme8
    fn inst_0xfe_cp_imm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.read_word(bus)?;
        self.cp_a_with(data);
        Ok(8)
    }
}

/// JP & JR
impl CPU {
    /// JP imme16
    fn inst_0xc3_jp_imm16(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let addr = self.read_dword(bus)?;
        self.jp(addr);
        Ok(16)
    }

    /// JP NZ, imme16
    fn inst_0xc2_jp_nz_imm16(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let jump_to = self.read_dword(bus)?;
        if !self.regs.zero_flag() {
            self.jp(jump_to);
            Ok(16)
        } else {
            Ok(12)
        }
    }

    /// JP Z, imme16
    fn inst_0xca_jp_z_imm16(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let jump_to = self.read_dword(bus)?;
        if self.regs.zero_flag() {
            self.jp(jump_to);
            Ok(16)
        } else {
            Ok(12)
        }
    }

    /// JP NC, imme16
    fn inst_0xd2_jp_nc_imm16(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let jump_to = self.read_dword(bus)?;
        if !self.regs.carry_flag() {
            self.jp(jump_to);
            Ok(16)
        } else {
            Ok(12)
        }
    }

    /// JP C, imme16
    fn inst_0xda_jp_c_imm16(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let jump_to = self.read_dword(bus)?;
        if self.regs.carry_flag() {
            self.jp(jump_to);
            Ok(16)
        } else {
            Ok(12)
        }
    }

    /// JP HL
    fn inst_0xe9_jp_hl(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let to = self.hl();
        self.jp(to);
        // 只花费1个机器周期，没有流水线停顿的惩罚
        Ok(4)
    }

    /// JR imme8
    fn inst_0x18_jr_imm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let offset = self.read_word(bus)?;
        self.jr(offset);
        Ok(12)
    }

    /// JR NZ, imme8
    fn inst_0x20_jr_nz_imm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let offset = self.read_word(bus)?;
        if !self.regs.zero_flag() {
            self.jr(offset);
            Ok(12)
        } else {
            Ok(8)
        }
    }

    /// JR Z, imme8
    fn inst_0x28_jr_z_imm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let offset = self.read_word(bus)?;
        if self.regs.zero_flag() {
            self.jr(offset);
            Ok(12)
        } else {
            Ok(8)
        }
    }

    /// JR NC, imme8
    fn inst_0x30_jr_nc_imm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let offset = self.read_word(bus)?;
        if !self.regs.carry_flag() {
            self.jr(offset);
            Ok(12)
        } else {
            Ok(8)
        }
    }

    /// JR C, imme8
    fn inst_0x38_jr_c_imm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let offset = self.read_word(bus)?;
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
        let data = self.bc();
        self.push_dword(bus, data)?;
        Ok(16)
    }

    /// PUSH DE
    fn inst_0xd5_push_de(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.de();
        self.push_dword(bus, data)?;
        Ok(16)
    }

    /// PUSH HL
    fn inst_0xe5_push_hl(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.hl();
        self.push_dword(bus, data)?;
        Ok(16)
    }

    /// PUSH AF
    fn inst_0xf5_push_af(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.af();
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
        *self.bc_mut() = data;
        Ok(12)
    }

    /// POP DE
    fn inst_0xd1_pop_de(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.pop_dword(bus)?;
        *self.de_mut() = data;
        Ok(12)
    }

    /// POP HL
    fn inst_0xe1_pop_hl(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.pop_dword(bus)?;
        *self.hl_mut() = data;
        Ok(12)
    }

    /// POP AF
    fn inst_0xf1_pop_af(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.pop_dword(bus)?;
        *self.af_mut() = data;
        Ok(12)
    }

    /// CALL imme16
    fn inst_0xcd_call_imm16(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let addr = self.read_dword(bus)?;
        let pc = self.pc();
        self.push_dword(bus, pc)?;
        self.jp(addr);
        Ok(24)
    }

    /// CALL NZ, imme16
    fn inst_0xc4_call_nz_imm16(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let jump_to = self.read_dword(bus)?;
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
        let jump_to = self.read_dword(bus)?;
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
        let jump_to = self.read_dword(bus)?;
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
        let jump_to = self.read_dword(bus)?;
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
        self.ime.enable();
        Ok(16)
    }

    /// RST 0x0000
    fn inst_0xc7_rst_0x0000(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        self.push_dword(bus, self.pc())?;
        self.jp(0x0000);
        Ok(16)
    }

    /// RST 0x0008
    fn inst_0xcf_rst_0x0008(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        self.push_dword(bus, self.pc())?;
        self.jp(0x0008);
        Ok(16)
    }

    /// RST 0x0010
    fn inst_0xd7_rst_0x0010(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        self.push_dword(bus, self.pc())?;
        self.jp(0x0010);
        Ok(16)
    }

    /// RST 0x0018
    fn inst_0xdf_rst_0x0018(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        self.push_dword(bus, self.pc())?;
        self.jp(0x0018);
        Ok(16)
    }

    /// RST 0x0020
    fn inst_0xe7_rst_0x0020(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        self.push_dword(bus, self.pc())?;
        self.jp(0x0020);
        Ok(16)
    }

    /// RST 0x0028
    fn inst_0xef_rst_0x0028(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        self.push_dword(bus, self.pc())?;
        self.jp(0x0028);
        Ok(16)
    }

    /// RST 0x0030
    fn inst_0xf7_rst_0x0030(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        self.push_dword(bus, self.pc())?;
        self.jp(0x0030);
        Ok(16)
    }

    /// RST 0x0038
    fn inst_0xff_rst_0x0038(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        self.push_dword(bus, self.pc())?;
        self.jp(0x0038);
        Ok(16)
    }
}

/// 算术逻辑运算指令
impl CPU {
    /// INC B
    fn inst_0x04_inc_b(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let b = self.b_mut();
        let result = b.wrapping_add(1);
        *b = result;
        self.zero_flag_mut().set_value(result == 0);
        self.negative_flag_mut().clear();
        self.half_carry_flag_mut().set_value((result & 0xF) == 0x0);
        Ok(4)
    }

    /// DEC B
    fn inst_0x05_dec_b(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let b = self.b_mut();
        let result = b.wrapping_sub(1);
        *b = result;
        self.zero_flag_mut().set_value(result == 0);
        self.negative_flag_mut().clear();
        self.half_carry_flag_mut().set_value((result & 0xF) == 0xF);
        Ok(4)
    }

    /// INC C
    fn inst_0x0c_inc_c(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let c = self.c_mut();
        let result = c.wrapping_add(1);
        *c = result;
        self.zero_flag_mut().set_value(result == 0);
        self.negative_flag_mut().clear();
        self.half_carry_flag_mut().set_value((result & 0xF) == 0x0);
        Ok(4)
    }

    /// DEC C
    fn inst_0x0d_dec_c(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let c = self.c_mut();
        let result = c.wrapping_sub(1);
        *c = result;
        self.zero_flag_mut().set_value(result == 0);
        self.negative_flag_mut().clear();
        self.half_carry_flag_mut().set_value((result & 0xF) == 0xF);
        Ok(4)
    }

    /// INC D
    fn inst_0x14_inc_d(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let d = self.d_mut();
        let result = d.wrapping_add(1);
        *d = result;
        self.zero_flag_mut().set_value(result == 0);
        self.negative_flag_mut().clear();
        self.half_carry_flag_mut().set_value((result & 0xF) == 0x0);
        Ok(4)
    }

    /// DEC D
    fn inst_0x15_dec_d(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let d = self.d_mut();
        let result = d.wrapping_sub(1);
        *d = result;
        self.zero_flag_mut().set_value(result == 0);
        self.negative_flag_mut().clear();
        self.half_carry_flag_mut().set_value((result & 0xF) == 0xF);
        Ok(4)
    }

    /// INC E
    fn inst_0x1c_inc_e(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let e = self.e_mut();
        let result = e.wrapping_add(1);
        *e = result;
        self.zero_flag_mut().set_value(result == 0);
        self.negative_flag_mut().clear();
        self.half_carry_flag_mut().set_value((result & 0xF) == 0x0);
        Ok(4)
    }

    /// DEC E
    fn inst_0x1d_dec_e(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let e = self.e_mut();
        let result = e.wrapping_sub(1);
        *e = result;
        self.zero_flag_mut().set_value(result == 0);
        self.negative_flag_mut().clear();
        self.half_carry_flag_mut().set_value((result & 0xF) == 0xF);
        Ok(4)
    }

    /// INC H
    fn inst_0x24_inc_h(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let h = self.h_mut();
        let result = h.wrapping_add(1);
        *h = result;
        self.zero_flag_mut().set_value(result == 0);
        self.negative_flag_mut().clear();
        self.half_carry_flag_mut().set_value((result & 0xF) == 0x0);
        Ok(4)
    }

    /// DEC HINC
    fn inst_0x25_dec_h(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let h = self.h_mut();
        let result = h.wrapping_sub(1);
        *h = result;
        self.zero_flag_mut().set_value(result == 0);
        self.negative_flag_mut().clear();
        self.half_carry_flag_mut().set_value((result & 0xF) == 0xF);
        Ok(4)
    }

    /// INC L
    fn inst_0x2c_inc_l(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let l = self.l_mut();
        let result = l.wrapping_add(1);
        *l = result;
        self.zero_flag_mut().set_value(result == 0);
        self.negative_flag_mut().clear();
        self.half_carry_flag_mut().set_value((result & 0xF) == 0x0);
        Ok(4)
    }

    /// DEC L
    fn inst_0x2d_dec_l(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let l = self.l_mut();
        let result = l.wrapping_sub(1);
        *l = result;
        self.zero_flag_mut().set_value(result == 0);
        self.negative_flag_mut().clear();
        self.half_carry_flag_mut().set_value((result & 0xF) == 0xF);
        Ok(4)
    }

    /// INC A
    fn inst_0x3c_inc_a(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let a = self.a_mut();
        let result = a.wrapping_add(1);
        *a = result;
        self.zero_flag_mut().set_value(result == 0);
        self.negative_flag_mut().clear();
        self.half_carry_flag_mut().set_value((result & 0xF) == 0x0);
        Ok(4)
    }

    /// DEC A
    fn inst_0x3d_dec_a(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let a = self.a_mut();
        let result = a.wrapping_sub(1);
        *a = result;
        self.zero_flag_mut().set_value(result == 0);
        self.negative_flag_mut().clear();
        self.half_carry_flag_mut().set_value((result & 0xF) == 0xF);
        Ok(4)
    }

    /// INC (HL)
    fn inst_0x34_inc_mhl(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let addr = self.hl();
        let data = bus.read(addr)? + 1;
        self.zero_flag_mut().set_value(data == 0);
        self.negative_flag_mut().clear();
        self.half_carry_flag_mut().set_value((data & 0xF) == 0x0);
        bus.write(addr, data)?;
        Ok(12)
    }

    /// DEC (HL)
    fn inst_0x35_dec_mhl(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let addr = self.hl();
        let data = bus.read(addr)? - 1;
        self.zero_flag_mut().set_value(data == 0);
        self.negative_flag_mut().clear();
        self.half_carry_flag_mut().set_value((data & 0xF) == 0xF);
        bus.write(addr, data)?;
        Ok(12)
    }

    /// INC BC
    fn inst_0x03_inc_bc(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let bc = self.bc_mut();
        *bc = bc.wrapping_add(1);
        Ok(8)
    }

    /// DEC BC
    fn inst_0x0b_dec_bc(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let bc = self.bc_mut();
        *bc = bc.wrapping_sub(1);
        Ok(8)
    }

    /// INC DE
    fn inst_0x13_inc_de(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let de = self.de_mut();
        *de = de.wrapping_add(1);
        Ok(8)
    }

    /// DEC DE
    fn inst_0x1b_dec_de(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let de = self.de_mut();
        *de = de.wrapping_sub(1);
        Ok(8)
    }

    /// INC HL
    fn inst_0x23_inc_hl(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let hl = self.hl_mut();
        *hl = hl.wrapping_add(1);
        Ok(8)
    }

    /// DEC HL
    fn inst_0x2b_dec_hl(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let hl = self.hl_mut();
        *hl = hl.wrapping_sub(1);
        Ok(8)
    }

    /// INC SP
    fn inst_0x33_inc_sp(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let sp = self.sp_mut();
        *sp = sp.wrapping_add(1);
        Ok(8)
    }

    /// DEC SP
    fn inst_0x3b_dec_sp(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let sp = self.sp_mut();
        *sp = sp.wrapping_sub(1);
        Ok(8)
    }

    fn add_a_with(&mut self, rhs: Word) {
        let lhs = self.a() as u32;
        let rhs = rhs as u32;
        let result = lhs.wrapping_add(rhs);
        self.negative_flag_mut().clear();
        self.zero_flag_mut().set_value(result & 0xFF == 0);
        self.half_carry_flag_mut()
            .set_value(lhs & 0xF + rhs & 0xF > 0xF);
        self.carry_flag_mut().set_value(result > 0xFF);
        *self.a_mut() = result as Word;
    }

    /// ADD A, B
    fn inst_0x80_add_a_b(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.add_a_with(self.b());
        Ok(4)
    }

    /// ADD A, C
    fn inst_0x81_add_a_c(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.add_a_with(self.c());
        Ok(4)
    }

    /// ADD A, D
    fn inst_0x82_add_a_d(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.add_a_with(self.d());
        Ok(4)
    }

    /// ADD A, E
    fn inst_0x83_add_a_e(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.add_a_with(self.e());
        Ok(4)
    }

    /// ADD A, H
    fn inst_0x84_add_a_h(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.add_a_with(self.h());
        Ok(4)
    }

    /// ADD A, L
    fn inst_0x85_add_a_l(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.add_a_with(self.l());
        Ok(4)
    }

    /// ADD A, A
    fn inst_0x87_add_a_a(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.add_a_with(self.a());
        Ok(4)
    }

    /// ADD A, imm8
    fn inst_0xc6_add_a_imm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.read_word(bus)?;
        self.add_a_with(data);
        Ok(8)
    }

    /// ADD A, (HL)
    fn inst_0x86_add_a_mhl(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = bus.read(self.hl())?;
        self.add_a_with(data);
        Ok(8)
    }

    fn add_hl_with(&mut self, rhs: DWord) {
        let lhs = self.hl() as u32;
        let rhs = rhs as u32;
        let result = lhs.wrapping_add(rhs);
        self.negative_flag_mut().clear();
        self.half_carry_flag_mut()
            .set_value(lhs & 0xFFF + rhs & 0xFFF > 0xFFF);
        self.carry_flag_mut().set_value(result > 0xFFFF);
        *self.hl_mut() = result as DWord;
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
        let imm8 = self.read_word(bus)? as i8 as DWord;
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

    fn inst_0x8f_adc_a_a(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.adc_a_with(self.a());
        Ok(4)
    }

    /// ADC A, imm8
    fn inst_0xce_add_a_imm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.read_word(bus)?;
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

    fn inst_0x97_sub_a_a(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        self.sub_a_with(self.a());
        Ok(4)
    }

    /// SUB A, imm8
    fn inst_0xd6_sub_a_imm8(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let data = self.read_word(bus)?;
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
        let data = self.read_word(bus)?;
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
        let data = self.read_word(bus)?;
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
        let data = self.read_word(bus)?;
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
        let data = self.read_word(bus)?;
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

    fn inst_0x27_daa(&mut self, _: &mut Bus) -> Result<ClockCycle> {
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

impl CPU {
    fn inst_0xcb_prefix_cb(&mut self, bus: &mut Bus) -> Result<ClockCycle> {
        let code = self.read_word(bus)?;
        let (inst, operand) = extended_inst_decode(code);
        match operand {
            OPERAND_B => {
                let val = self.b();
                let flag = self.f();
                let (new_val, new_flag) = inst(val, flag);
                *self.b_mut() = new_val;
                *self.f_mut() = new_flag;
                Ok(8)
            }
            OPERAND_C => {
                let val = self.c();
                let flag = self.f();
                let (new_val, new_flag) = inst(val, flag);
                *self.c_mut() = new_val;
                *self.f_mut() = new_flag;
                Ok(8)
            }
            OPERAND_D => {
                let val = self.d();
                let flag = self.f();
                let (new_val, new_flag) = inst(val, flag);
                *self.d_mut() = new_val;
                *self.f_mut() = new_flag;
                Ok(8)
            }
            OPERAND_E => {
                let val = self.e();
                let flag = self.f();
                let (new_val, new_flag) = inst(val, flag);
                *self.e_mut() = new_val;
                *self.f_mut() = new_flag;
                Ok(8)
            }
            OPERAND_H => {
                let val = self.h();
                let flag = self.f();
                let (new_val, new_flag) = inst(val, flag);
                *self.h_mut() = new_val;
                *self.f_mut() = new_flag;
                Ok(8)
            }
            OPERAND_L => {
                let val = self.l();
                let flag = self.f();
                let (new_val, new_flag) = inst(val, flag);
                *self.l_mut() = new_val;
                *self.f_mut() = new_flag;
                Ok(8)
            }
            OPERAND_MHL => {
                let addr = self.hl();
                let val = bus.read(addr)?;
                let flag = self.f();
                let (new_val, new_flag) = inst(val, flag);
                bus.write(addr, new_val)?;
                *self.f_mut() = new_flag;
                Ok(16)
            }
            OPERAND_A => {
                let val = self.a();
                let flag = self.f();
                let (new_val, new_flag) = inst(val, flag);
                *self.a_mut() = new_val;
                *self.f_mut() = new_flag;
                Ok(8)
            }
            _ => unreachable!(),
        }
    }

    fn inst_0x07_rlca(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let a = self.a();
        let carry = a.at(7);
        let val = a << 1 | carry;
        *self.a_mut() = val;
        self.zero_flag_mut().clear();
        self.carry_flag_mut().set_value(carry != 0);
        Ok(4)
    }

    fn inst_0x0f_rrca(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let a = self.a();
        let carry = a.at(0);
        let val = a >> 1 | carry << 7;
        *self.a_mut() = val;
        self.zero_flag_mut().clear();
        self.carry_flag_mut().set_value(carry != 0);
        Ok(4)
    }

    fn inst_0x17_rla(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let a = self.a();
        let new_carry = a.at(7);
        let new_val = a << 1 | if self.carry_flag() { 1 } else { 0 };
        *self.a_mut() = new_val;
        self.zero_flag_mut().clear();
        self.carry_flag_mut().set_value(new_carry != 0);
        Ok(4)
    }

    fn inst_0x1f_rra(&mut self, _: &mut Bus) -> Result<ClockCycle> {
        let a = self.a();
        let new_carry = a.at(0);
        let new_val = a >> 1 | if self.carry_flag() { 1 } else { 0 } << 7;
        *self.a_mut() = new_val;
        self.zero_flag_mut().clear();
        self.carry_flag_mut().set_value(new_carry != 0);
        Ok(4)
    }
}
