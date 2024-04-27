use crate::{types::Word, utils::bits::Bits};

const SET: Word = 0b1;
const UNSET: Word = 0b0;
const ZERO: Word = 7;
const NEGATIVE: Word = 6;
const HALF_CARRY: Word = 5;
const CARRY: Word = 4;

const RLC: u8 = 0b00000;
const RRC: u8 = 0b00001;
const RL: u8 = 0b00010;
const RR: u8 = 0b00011;
const SLA: u8 = 0b00100;
const SRA: u8 = 0b00101;
const SWAP: u8 = 0b00110;
const SRL: u8 = 0b00111;
const BIT0: u8 = 0b01000;
const BIT1: u8 = 0b01001;
const BIT2: u8 = 0b01010;
const BIT3: u8 = 0b01011;
const BIT4: u8 = 0b01100;
const BIT5: u8 = 0b01101;
const BIT6: u8 = 0b01110;
const BIT7: u8 = 0b01111;
const RES0: u8 = 0b10000;
const RES1: u8 = 0b10001;
const RES2: u8 = 0b10010;
const RES3: u8 = 0b10011;
const RES4: u8 = 0b10100;
const RES5: u8 = 0b10101;
const RES6: u8 = 0b10110;
const RES7: u8 = 0b10111;
const SET0: u8 = 0b11000;
const SET1: u8 = 0b11001;
const SET2: u8 = 0b11010;
const SET3: u8 = 0b11011;
const SET4: u8 = 0b11100;
const SET5: u8 = 0b11101;
const SET6: u8 = 0b11110;
const SET7: u8 = 0b11111;

const OPERAND_B: u8 = 0b000;
const OPERAND_C: u8 = 0b001;
const OPERAND_D: u8 = 0b010;
const OPERAND_E: u8 = 0b011;
const OPERAND_H: u8 = 0b100;
const OPERAND_L: u8 = 0b101;
const OPERAND_MHL: u8 = 0b110;
const OPERAND_A: u8 = 0b111;

/// (旧值, 旧标志位) -> (新值, 新标志位)
pub type ExtendedInst = fn(Word, Word) -> (Word, Word);

trait Flag {
    fn zero(self) -> Word;
    fn negative(self) -> Word;
    fn half_carry(self) -> Word;
    fn carry(self) -> Word;
    fn from_flags(zero: Word, negative: Word, half_carry: Word, carry: Word) -> Self;
    fn empty() -> Self;
    fn set_zero(self) -> Self;
    fn set_negative(self) -> Self;
    fn set_half_carry(self) -> Self;
    fn set_carry(self) -> Self;
    fn clear_zero(self) -> Self;
    fn clear_negative(self) -> Self;
    fn clear_half_carry(self) -> Self;
    fn clear_carry(self) -> Self;
    fn set_zero_with(self, val: bool) -> Self;
    fn set_negative_with(self, val: bool) -> Self;
    fn set_half_carry_with(self, val: bool) -> Self;
    fn set_carry_with(self, val: bool) -> Self;
}

impl Flag for Word {
    #[inline]
    fn zero(self) -> Word {
        if self & (1 << ZERO) != 0 {
            SET
        } else {
            UNSET
        }
    }

    #[inline]
    fn negative(self) -> Word {
        if self & (1 << NEGATIVE) != 0 {
            SET
        } else {
            UNSET
        }
    }

    #[inline]
    fn half_carry(self) -> Word {
        if self & (1 << HALF_CARRY) != 0 {
            SET
        } else {
            UNSET
        }
    }

    #[inline]
    fn carry(self) -> Word {
        if self & (1 << CARRY) != 0 {
            SET
        } else {
            UNSET
        }
    }

    #[inline]
    fn from_flags(zero: Word, negative: Word, half_carry: Word, carry: Word) -> Self {
        zero << ZERO | negative << NEGATIVE | half_carry << HALF_CARRY | carry << CARRY
    }

    #[inline]
    fn empty() -> Self {
        0
    }

    #[inline]
    fn set_zero(self) -> Self {
        self | (1 << ZERO)
    }

    #[inline]
    fn set_negative(self) -> Self {
        self | (1 << NEGATIVE)
    }

    #[inline]
    fn set_half_carry(self) -> Self {
        self | (1 << HALF_CARRY)
    }

    #[inline]
    fn set_carry(self) -> Self {
        self | (1 << CARRY)
    }

    #[inline]
    fn clear_zero(self) -> Self {
        self & !(1 << ZERO)
    }

    #[inline]
    fn clear_negative(self) -> Self {
        self & !(1 << NEGATIVE)
    }

    #[inline]
    fn clear_half_carry(self) -> Self {
        self & !(1 << HALF_CARRY)
    }

    #[inline]
    fn clear_carry(self) -> Self {
        self & !(1 << CARRY)
    }

    #[inline]
    fn set_zero_with(self, val: bool) -> Self {
        if val {
            self.set_zero()
        } else {
            self.clear_zero()
        }
    }

    #[inline]
    fn set_negative_with(self, val: bool) -> Self {
        if val {
            self.set_negative()
        } else {
            self.clear_negative()
        }
    }

    #[inline]
    fn set_half_carry_with(self, val: bool) -> Self {
        if val {
            self.set_half_carry()
        } else {
            self.clear_half_carry()
        }
    }

    #[inline]
    fn set_carry_with(self, val: bool) -> Self {
        if val {
            self.set_carry()
        } else {
            self.clear_carry()
        }
    }
}

fn rlc(val: Word, _: Word) -> (Word, Word) {
    let bits = Bits::from(val);
    let carry = bits.at(7);
    let val = bits.into_word() << 1 | carry;
    let flag = Word::empty()
        .set_zero_with(val == 0)
        .set_carry_with(carry != 0);
    (val, flag)
}

fn rrc(val: Word, _: Word) -> (Word, Word) {
    let val = Bits::from(val);
    let carry = val.at(0);
    let val = val.into_word() >> 1 | carry << 7;
    let flag = Word::empty()
        .set_zero_with(val == 0)
        .set_carry_with(carry != 0);
    (val, flag)
}

fn rl(val: Word, flag: Word) -> (Word, Word) {
    let carry = flag >> CARRY;
}
