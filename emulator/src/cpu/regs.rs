use crate::{
    types::{Addr, DWord, Word},
    utils::bit_proxy::BitProxy,
};
use std::default::Default;

/// https://gbdev.io/pandocs/CPU_Registers_and_Flags.html#registers
/// --------
/// | A | F |
/// | - | - |
/// | B | C |
/// | - | - |
/// | D | E |
/// | - | - |
/// | H | L |
/// --------
/// |  SP   |
/// |-------|
/// |  PC   |
/// ---------
///
/// 8个8位寄存器(可以组成4个16位寄存器) 2个16位寄存器
#[derive(Default)]
pub struct Regs([DWord; 6]);

impl Regs {
    pub const A: usize = 0;
    /// F（flags）寄存器用于存储CPU在运行过程中产生的各种位，其只有高位的4个比特有效，低位的4个比特永远是0
    pub const F: usize = 1;
    pub const B: usize = 2;
    pub const C: usize = 3;
    pub const D: usize = 4;
    pub const E: usize = 5;
    pub const H: usize = 6;
    pub const L: usize = 7;

    pub const AF: usize = 0;
    pub const BC: usize = 1;
    pub const DE: usize = 2;
    pub const HL: usize = 3;
    pub const SP: usize = 4;
    pub const PC: usize = 5;

    /// 当运算结果为0时设置为1，否则设置为0。
    const ZERO_FLAG: u8 = 7;
    /// 当运算为减法时设置为1，否则设置为0。
    const NEGATIVE_FLAG: u8 = 6;
    /// 当运算出现了比特3与比特4之间的进位或退位时设置为1，否则设置为0。
    const HALF_CARRY_FLAG: u8 = 5;
    /// 当运算出现了向上或者向下溢出时设置为1，否则设置为0
    const CARRY_FLAG: u8 = 4;

    #[inline]
    /// DMG初版的初始状态
    pub fn new() -> Self {
        Self([0x0001, 0x13FF, 0xC100, 0x0384, 0xFFFE, 0x0100])
    }

    #[inline]
    fn as_double_word_registers(&self) -> &[DWord; 6] {
        &self.0
    }

    #[inline]
    fn as_double_word_registers_mut(&mut self) -> &mut [DWord; 6] {
        &mut self.0
    }

    #[inline]
    fn as_single_word_registers(&self) -> &[Word; 8] {
        unsafe { &*(self as *const Self as *const _) }
    }

    #[inline]
    fn as_single_word_registers_mut(&mut self) -> &mut [Word; 8] {
        unsafe { &mut *(self as *mut Self as *mut _) }
    }

    #[inline]
    pub fn a(&self) -> Word {
        self.as_single_word_registers()[Self::A]
    }

    #[inline]
    pub fn a_mut<'a>(&'a mut self) -> &'a mut Word {
        &mut self.as_single_word_registers_mut()[Self::A]
    }

    #[inline]
    pub fn b(&self) -> Word {
        self.as_single_word_registers()[Self::B]
    }

    #[inline]
    pub fn b_mut<'a>(&'a mut self) -> &'a mut Word {
        &mut self.as_single_word_registers_mut()[Self::B]
    }

    #[inline]
    pub fn c(&self) -> Word {
        self.as_single_word_registers()[Self::C]
    }

    #[inline]
    pub fn c_mut<'a>(&'a mut self) -> &'a mut Word {
        &mut self.as_single_word_registers_mut()[Self::C]
    }

    #[inline]
    pub fn d(&self) -> Word {
        self.as_single_word_registers()[Self::D]
    }

    #[inline]
    pub fn d_mut<'a>(&'a mut self) -> &'a mut Word {
        &mut self.as_single_word_registers_mut()[Self::D]
    }

    #[inline]
    pub fn e(&self) -> Word {
        self.as_single_word_registers()[Self::E]
    }

    #[inline]
    pub fn e_mut<'a>(&'a mut self) -> &'a mut Word {
        &mut self.as_single_word_registers_mut()[Self::E]
    }

    #[inline]
    pub fn f(&self) -> Word {
        self.as_single_word_registers()[Self::F]
    }

    #[inline]
    pub fn f_mut<'a>(&'a mut self) -> &'a mut Word {
        &mut self.as_single_word_registers_mut()[Self::F]
    }

    #[inline]
    pub fn h(&self) -> Word {
        self.as_single_word_registers()[Self::H]
    }

    #[inline]
    pub fn h_mut<'a>(&'a mut self) -> &'a mut Word {
        &mut self.as_single_word_registers_mut()[Self::H]
    }

    #[inline]
    pub fn l(&self) -> Word {
        self.as_single_word_registers()[Self::L]
    }

    #[inline]
    pub fn l_mut<'a>(&'a mut self) -> &'a mut Word {
        &mut self.as_single_word_registers_mut()[Self::L]
    }

    #[inline]
    pub fn af(&self) -> DWord {
        self.as_double_word_registers()[Self::AF]
    }

    #[inline]
    pub fn af_mut<'a>(&'a mut self) -> &'a mut DWord {
        &mut self.as_double_word_registers_mut()[Self::AF]
    }

    #[inline]
    pub fn bc(&self) -> DWord {
        self.as_double_word_registers()[Self::BC]
    }

    #[inline]
    pub fn bc_mut<'a>(&'a mut self) -> &'a mut DWord {
        &mut self.as_double_word_registers_mut()[Self::BC]
    }

    #[inline]
    pub fn de(&self) -> DWord {
        self.as_double_word_registers()[Self::DE]
    }

    #[inline]
    pub fn de_mut<'a>(&'a mut self) -> &'a mut DWord {
        &mut self.as_double_word_registers_mut()[Self::DE]
    }

    #[inline]
    pub fn hl(&self) -> DWord {
        self.as_double_word_registers()[Self::HL]
    }

    #[inline]
    pub fn hl_mut<'a>(&'a mut self) -> &'a mut DWord {
        &mut self.as_double_word_registers_mut()[Self::HL]
    }

    #[inline]
    pub fn sp(&self) -> Addr {
        self.as_double_word_registers()[Self::SP]
    }

    #[inline]
    pub fn sp_mut<'a>(&'a mut self) -> &'a mut Addr {
        &mut self.as_double_word_registers_mut()[Self::SP]
    }

    #[inline]
    pub fn pc(&self) -> Addr {
        self.as_double_word_registers()[Self::PC]
    }

    #[inline]
    pub fn pc_mut<'a>(&'a mut self) -> &'a mut Addr {
        &mut self.as_double_word_registers_mut()[Self::PC]
    }

    #[inline]
    pub fn dw_reg(&self, idx: usize) -> DWord {
        self.as_double_word_registers()[idx]
    }

    #[inline]
    pub fn dw_reg_mut<'a>(&'a mut self, idx: usize) -> &'a mut DWord {
        &mut self.as_double_word_registers_mut()[idx]
    }

    #[inline]
    pub fn w_reg(&self, idx: usize) -> Word {
        self.as_single_word_registers()[idx]
    }

    #[inline]
    pub fn w_reg_mut<'a>(&'a mut self, idx: usize) -> &'a mut Word {
        &mut self.as_single_word_registers_mut()[idx]
    }

    #[inline]
    pub fn zero_flag(&self) -> bool {
        (self.f() & 1 << Self::ZERO_FLAG) != 0
    }

    #[inline]
    pub fn zero_flag_mut<'a>(&'a mut self) -> BitProxy<'a> {
        BitProxy::new(self.f_mut(), Self::ZERO_FLAG)
    }

    #[inline]
    pub fn negative_flag(&self) -> bool {
        (self.f() & 1 << Self::NEGATIVE_FLAG) != 0
    }

    #[inline]
    pub fn negative_flag_mut<'a>(&'a mut self) -> BitProxy<'a> {
        BitProxy::new(self.f_mut(), Self::NEGATIVE_FLAG)
    }

    #[inline]
    pub fn half_carry_flag(&self) -> bool {
        (self.f() & 1 << Self::HALF_CARRY_FLAG) != 0
    }

    #[inline]
    pub fn half_carry_flag_mut<'a>(&'a mut self) -> BitProxy<'a> {
        BitProxy::new(self.f_mut(), Self::HALF_CARRY_FLAG)
    }

    #[inline]
    pub fn carry_flag(&self) -> bool {
        (self.f() & 1 << Self::CARRY_FLAG) != 0
    }

    #[inline]
    pub fn carry_flag_mut<'a>(&'a mut self) -> BitProxy<'a> {
        BitProxy::new(self.f_mut(), Self::CARRY_FLAG)
    }

    #[inline]
    pub fn reset(&mut self) {
        self.0 = [0; 6];
    }
}
