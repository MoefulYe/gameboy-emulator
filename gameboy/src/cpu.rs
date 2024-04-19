use std::default::{self, Default};

use crate::types::Addr;

/// 寄存器
/// --------
/// | A | F |
/// | - | - |
/// | B | C |
/// | - | - |
/// | D | E |
/// | - | - |
/// | H | L |
/// --------
///
/// ---------
/// |  SP   |
/// |-------|
/// |  PC   |
/// ---------
///
/// 8个8位寄存器(可以组成4个16位寄存器) 2个16位寄存器
#[derive(Default)]
pub struct Regs([u16; 4 + 2]);

impl Regs {
    const A: usize = 0;
    const F: usize = 1;
    const B: usize = 2;
    const C: usize = 3;
    const D: usize = 4;
    const E: usize = 5;
    const H: usize = 6;
    const L: usize = 7;
    const AF: usize = 0;
    const BC: usize = 1;
    const DE: usize = 2;
    const HL: usize = 3;
    const SP: usize = 4;
    const PC: usize = 5;

    #[inline]
    pub fn new() {
        Default::default()
    }

    #[inline]
    fn as_16bit_registers(&self) -> &[u16; 6] {
        unsafe { &*(self as *const Self as *const [u16; 6]) }
    }

    #[inline]
    fn as_16bit_registers_mut(&mut self) -> &mut [u16; 6] {
        unsafe { &mut *(self as *mut Self as *mut [u16; 6]) }
    }

    #[inline]
    fn as_8bit_registers(&self) -> &[u8; 8] {
        unsafe { &*(self as *const Self as *const [u8; 8]) }
    }

    #[inline]
    fn as_8bit_registers_mut(&mut self) -> &mut [u8; 8] {
        unsafe { &mut *(self as *mut Self as *mut [u8; 8]) }
    }

    #[inline]
    pub fn a(&self) -> u8 {
        self.as_8bit_registers()[Self::A]
    }

    #[inline]
    pub fn a_mut<'a>(&'a mut self) -> &'a mut u8 {
        &mut self.as_8bit_registers_mut()[Self::A]
    }

    #[inline]
    pub fn b(&self) -> u8 {
        self.as_8bit_registers()[Self::B]
    }

    #[inline]
    pub fn b_mut<'a>(&'a mut self) -> &'a mut u8 {
        &mut self.as_8bit_registers_mut()[Self::B]
    }

    #[inline]
    pub fn c(&self) -> u8 {
        self.as_8bit_registers()[Self::C]
    }

    #[inline]
    pub fn c_mut<'a>(&'a mut self) -> &'a mut u8 {
        &mut self.as_8bit_registers_mut()[Self::C]
    }

    #[inline]
    pub fn d(&self) -> u8 {
        self.as_8bit_registers()[Self::D]
    }

    #[inline]
    pub fn d_mut<'a>(&'a mut self) -> &'a mut u8 {
        &mut self.as_8bit_registers_mut()[Self::D]
    }

    #[inline]
    pub fn e(&self) -> u8 {
        self.as_8bit_registers()[Self::E]
    }

    #[inline]
    pub fn e_mut<'a>(&'a mut self) -> &'a mut u8 {
        &mut self.as_8bit_registers_mut()[Self::E]
    }

    #[inline]
    pub fn f(&self) -> u8 {
        self.as_8bit_registers()[Self::F]
    }

    #[inline]
    pub fn f_mut<'a>(&'a mut self) -> &'a mut u8 {
        &mut self.as_8bit_registers_mut()[Self::F]
    }

    #[inline]
    pub fn h(&self) -> u8 {
        self.as_8bit_registers()[Self::H]
    }

    #[inline]
    pub fn h_mut<'a>(&'a mut self) -> &'a mut u8 {
        &mut self.as_8bit_registers_mut()[Self::H]
    }

    #[inline]
    pub fn l(&self) -> u8 {
        self.as_8bit_registers()[Self::L]
    }

    #[inline]
    pub fn l_mut<'a>(&'a mut self) -> &'a mut u8 {
        &mut self.as_8bit_registers_mut()[Self::L]
    }

    #[inline]
    pub fn af(&self) -> u16 {
        self.as_16bit_registers()[Self::AF]
    }

    #[inline]
    pub fn af_mut<'a>(&'a mut self) -> &'a mut u16 {
        &mut self.as_16bit_registers_mut()[Self::AF]
    }

    #[inline]
    pub fn bc(&self) -> u16 {
        self.as_16bit_registers()[Self::BC]
    }

    #[inline]
    pub fn bc_mut<'a>(&'a mut self) -> &'a mut u16 {
        &mut self.as_16bit_registers_mut()[Self::BC]
    }

    #[inline]
    pub fn de(&self) -> u16 {
        self.as_16bit_registers()[Self::DE]
    }

    #[inline]
    pub fn de_mut<'a>(&'a mut self) -> &'a mut u16 {
        &mut self.as_16bit_registers_mut()[Self::DE]
    }

    #[inline]
    pub fn hl(&self) -> u16 {
        self.as_16bit_registers()[Self::HL]
    }

    #[inline]
    pub fn hl_mut<'a>(&'a mut self) -> &'a mut u16 {
        &mut self.as_16bit_registers_mut()[Self::HL]
    }

    #[inline]
    pub fn sp(&self) -> Addr {
        self.as_16bit_registers()[Self::SP]
    }

    #[inline]
    pub fn sp_mut<'a>(&'a mut self) -> &'a mut Addr {
        &mut self.as_16bit_registers_mut()[Self::SP]
    }

    #[inline]
    pub fn pc(&self) -> Addr {
        self.as_16bit_registers()[Self::PC]
    }

    #[inline]
    pub fn pc_mut<'a>(&'a mut self) -> &'a mut Addr {
        &mut self.as_16bit_registers_mut()[Self::PC]
    }
}

pub struct CPU {}
