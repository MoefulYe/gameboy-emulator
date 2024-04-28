use crate::{types::Word, utils::bits::BitMap};

const ZERO: Word = 7;
const NEGATIVE: Word = 6;
const HALF_CARRY: Word = 5;
const CARRY: Word = 4;

pub const OPERAND_B: u8 = 0b000;
pub const OPERAND_C: u8 = 0b001;
pub const OPERAND_D: u8 = 0b010;
pub const OPERAND_E: u8 = 0b011;
pub const OPERAND_H: u8 = 0b100;
pub const OPERAND_L: u8 = 0b101;
pub const OPERAND_MHL: u8 = 0b110;
pub const OPERAND_A: u8 = 0b111;
pub type Operand = u8;

/// (旧值, 旧标志位) -> (新值, 新标志位)
pub type ExtendedInst = fn(Word, Word) -> (Word, Word);
const EXTENDED_INSTS: &[ExtendedInst; 32] = &[
    cb_inst_0b00000_rlc,
    cb_inst_0b00001_rrc,
    cb_inst_0b00010_rl,
    cb_inst_0b00011_rr,
    cb_inst_0b00100_sla,
    cb_inst_0b00101_sra,
    cb_inst_0b00110_swap,
    cb_inst_0b00111_srl,
    cb_inst_0b01000_bit0,
    cb_inst_0b01001_bit1,
    cb_inst_0b01010_bit2,
    cb_inst_0b01011_bit3,
    cb_inst_0b01100_bit4,
    cb_inst_0b01101_bit5,
    cb_inst_0b01110_bit6,
    cb_inst_0b01111_bit7,
    cb_inst_0b10000_res0,
    cb_inst_0b10001_res1,
    cb_inst_0b10010_res2,
    cb_inst_0b10011_res3,
    cb_inst_0b10100_res4,
    cb_inst_0b10101_res5,
    cb_inst_0b10110_res6,
    cb_inst_0b10111_res7,
    cb_inst_0b11000_set0,
    cb_inst_0b11001_set1,
    cb_inst_0b11010_set2,
    cb_inst_0b11011_set3,
    cb_inst_0b11100_set4,
    cb_inst_0b11101_set5,
    cb_inst_0b11110_set6,
    cb_inst_0b11111_set7,
];

const OPCODE_WIDTH: usize = 5;
const OPCODE_MASK: Word = 0b1111_1000;
const OPCODE_SHIFT: usize = 3;
const OPERAND_WIDTH: usize = 3;
const OPERAND_MASK: Word = 0b0000_0111;
pub fn extended_inst_decode(word: Word) -> (ExtendedInst, Operand) {
    let opcode = (word & OPCODE_MASK) >> OPCODE_SHIFT;
    let operand = word & OPERAND_MASK;
    let inst = *unsafe { EXTENDED_INSTS.get_unchecked(opcode as usize) };
    (inst, operand)
}

fn cb_inst_0b00000_rlc(val: Word, _: Word) -> (Word, Word) {
    let carry = val.at(7);
    let val = val << 1 | carry;
    let flag = Word::empty()
        .set_at_with(CARRY, carry != 0)
        .set_at_with(ZERO, val == 0);
    (val, flag)
}

fn cb_inst_0b00001_rrc(val: Word, _: Word) -> (Word, Word) {
    let carry = val.at(0);
    let val = val >> 1 | carry << 7;
    let flag = Word::empty()
        .set_at_with(CARRY, carry != 0)
        .set_at_with(ZERO, val == 0);
    (val, flag)
}

fn cb_inst_0b00010_rl(val: Word, flag: Word) -> (Word, Word) {
    let new_carry = val.at(7);
    let new_val = val << 1 | flag.at(CARRY);
    let flag = Word::empty()
        .set_at_with(CARRY, new_carry != 0)
        .set_at_with(ZERO, new_val == 0);
    (new_val, flag)
}

fn cb_inst_0b00011_rr(val: Word, flag: Word) -> (Word, Word) {
    let new_carry = val.at(0);
    let new_val = val >> 1 | flag.at(CARRY) << 7;
    let flag = Word::empty()
        .set_at_with(CARRY, new_carry != 0)
        .set_at_with(ZERO, new_val == 0);
    (new_val, flag)
}

fn cb_inst_0b00100_sla(val: Word, _: Word) -> (Word, Word) {
    let carry = val.at(7);
    let val = val << 1;
    let flag = Word::empty()
        .set_at_with(CARRY, carry != 0)
        .set_at_with(ZERO, val == 0);
    (val, flag)
}

fn cb_inst_0b00101_sra(val: Word, _: Word) -> (Word, Word) {
    let carry = val.at(0);
    let val = (val >> 1) | (val & 0b1000_0000);
    let flag = Word::empty()
        .set_at_with(CARRY, carry != 0)
        .set_at_with(ZERO, val == 0);
    (val, flag)
}

fn cb_inst_0b00110_swap(val: Word, _: Word) -> (Word, Word) {
    let high = val >> 4;
    let low = val & 0b0000_1111;
    let val = low << 4 | high;
    let flag = Word::empty().set_at_with(ZERO, val != 0);
    (val, flag)
}

fn cb_inst_0b00111_srl(val: Word, _: Word) -> (Word, Word) {
    let carry = val.at(0);
    let val = val >> 1;
    let flag = Word::empty()
        .set_at_with(CARRY, carry != 0)
        .set_at_with(ZERO, val == 0);
    (val, flag)
}

fn cb_inst_0b01000_bit0(val: Word, flag: Word) -> (Word, Word) {
    let bit = val.at(0);
    let flag = flag
        .set_at_with(ZERO, bit != 0)
        .clear_at(NEGATIVE)
        .set_at(HALF_CARRY);
    (val, flag)
}

fn cb_inst_0b01001_bit1(val: Word, flag: Word) -> (Word, Word) {
    let bit = val.at(1);
    let flag = flag
        .set_at_with(ZERO, bit != 0)
        .clear_at(NEGATIVE)
        .set_at(HALF_CARRY);
    (val, flag)
}

fn cb_inst_0b01010_bit2(val: Word, flag: Word) -> (Word, Word) {
    let bit = val.at(2);
    let flag = flag
        .set_at_with(ZERO, bit != 0)
        .clear_at(NEGATIVE)
        .set_at(HALF_CARRY);
    (val, flag)
}

fn cb_inst_0b01011_bit3(val: Word, flag: Word) -> (Word, Word) {
    let bit = val.at(3);
    let flag = flag
        .set_at_with(ZERO, bit != 0)
        .clear_at(NEGATIVE)
        .set_at(HALF_CARRY);
    (val, flag)
}

fn cb_inst_0b01100_bit4(val: Word, flag: Word) -> (Word, Word) {
    let bit = val.at(4);
    let flag = flag
        .set_at_with(ZERO, bit != 0)
        .clear_at(NEGATIVE)
        .set_at(HALF_CARRY);
    (val, flag)
}

fn cb_inst_0b01101_bit5(val: Word, flag: Word) -> (Word, Word) {
    let bit = val.at(5);
    let flag = flag
        .set_at_with(ZERO, bit != 0)
        .clear_at(NEGATIVE)
        .set_at(HALF_CARRY);
    (val, flag)
}

fn cb_inst_0b01110_bit6(val: Word, flag: Word) -> (Word, Word) {
    let bit = val.at(6);
    let flag = flag
        .set_at_with(ZERO, bit != 0)
        .clear_at(NEGATIVE)
        .set_at(HALF_CARRY);
    (val, flag)
}

fn cb_inst_0b01111_bit7(val: Word, flag: Word) -> (Word, Word) {
    let bit = val.at(7);
    let flag = flag
        .set_at_with(ZERO, bit != 0)
        .clear_at(NEGATIVE)
        .set_at(HALF_CARRY);
    (val, flag)
}

fn cb_inst_0b10000_res0(val: Word, flag: Word) -> (Word, Word) {
    (val & 0b1111_1110, flag)
}

fn cb_inst_0b10001_res1(val: Word, flag: Word) -> (Word, Word) {
    (val & 0b1111_1101, flag)
}

fn cb_inst_0b10010_res2(val: Word, flag: Word) -> (Word, Word) {
    (val & 0b1111_1011, flag)
}

fn cb_inst_0b10011_res3(val: Word, flag: Word) -> (Word, Word) {
    (val & 0b1111_0111, flag)
}

fn cb_inst_0b10100_res4(val: Word, flag: Word) -> (Word, Word) {
    (val & 0b1110_1111, flag)
}

fn cb_inst_0b10101_res5(val: Word, flag: Word) -> (Word, Word) {
    (val & 0b1101_1111, flag)
}

fn cb_inst_0b10110_res6(val: Word, flag: Word) -> (Word, Word) {
    (val & 0b1011_1111, flag)
}

fn cb_inst_0b10111_res7(val: Word, flag: Word) -> (Word, Word) {
    (val & 0b0111_1111, flag)
}

fn cb_inst_0b11000_set0(val: Word, flag: Word) -> (Word, Word) {
    (val | 0b0000_0001, flag)
}

fn cb_inst_0b11001_set1(val: Word, flag: Word) -> (Word, Word) {
    (val | 0b0000_0010, flag)
}

fn cb_inst_0b11010_set2(val: Word, flag: Word) -> (Word, Word) {
    (val | 0b0000_0100, flag)
}

fn cb_inst_0b11011_set3(val: Word, flag: Word) -> (Word, Word) {
    (val | 0b0000_1000, flag)
}

fn cb_inst_0b11100_set4(val: Word, flag: Word) -> (Word, Word) {
    (val | 0b0001_0000, flag)
}

fn cb_inst_0b11101_set5(val: Word, flag: Word) -> (Word, Word) {
    (val | 0b0010_0000, flag)
}

fn cb_inst_0b11110_set6(val: Word, flag: Word) -> (Word, Word) {
    (val | 0b0100_0000, flag)
}

fn cb_inst_0b11111_set7(val: Word, flag: Word) -> (Word, Word) {
    (val | 0b1000_0000, flag)
}
