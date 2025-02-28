use super::{BusDevice, Reset};
use crate::output::audio::AudioOutput;
use crate::types::{Addr, Word};

mod chan1;
mod chan2;
mod chan3;
mod chan4;
mod modulation;

pub const APU_ADDR_LOW_BOUND: Addr = 0xFF10;
pub const APU_ADDR_HIGH_BOUND_INCLUDED: Addr = 0xFF26;

// Channel 1: Sweep
const REG_NR10_ADDR: Addr = 0xFF10;
// Channel 1: Sound Length / Wave Pattern Duty
const REG_NR11_ADDR: Addr = 0xFF11;
// Channel 1: Volume Envelope
const REG_NR12_ADDR: Addr = 0xFF12;
// Channel 1: Frequency lo data
const REG_NR13_ADDR: Addr = 0xFF13;
// Channel 1: Restart / Frequency hi data
const REG_NR14_ADDR: Addr = 0xFF14;
// Channel 2: Sound Length / Wave Pattern Duty
const REG_NR21_ADDR: Addr = 0xFF16;
// Channel 2: Volume Envelope
const REG_NR22_ADDR: Addr = 0xFF17;
// Channel 2: Frequency lo data
const REG_NR23_ADDR: Addr = 0xFF18;
// Channel 2: Restart / Frequency hi data
const REG_NR24_ADDR: Addr = 0xFF19;
// Channel 3: Sound on / off
const REG_NR30_ADDR: Addr = 0xFF1A;
// Channel 3: Sound length
const REG_NR31_ADDR: Addr = 0xFF1B;
// Channel 3: Volume
const REG_NR32_ADDR: Addr = 0xFF1C;
// Channel 3: Frequency lo data
const REG_NR33_ADDR: Addr = 0xFF1D;
// Channel 3: Restart / Frequency high data
const REG_NR34_ADDR: Addr = 0xFF1E;
// Channel 3: Wave pattern ram = 32 x 4bit
const WAVE_PATTERN_RAM_START: Addr = 0xFF30;
const WAVE_PATTERN_RAM_END: Addr = 0xFF3F;
// Channel 4: Sound Length
const REG_NR41_ADDR: Addr = 0xFF20;
// Channel 4: Volume Envelope
const REG_NR42_ADDR: Addr = 0xFF21;
// Channel 4: Polynomial counter
const REG_NR43_ADDR: Addr = 0xFF22;
// Channel 4: Restart / initial length
const REG_NR44_ADDR: Addr = 0xFF23;
// Sound controller: Channel control / ON-OFF / Volume
const REG_NR50_ADDR: Addr = 0xFF24;
// Sound controller: Selection of Sound output terminal
const REG_NR51_ADDR: Addr = 0xFF25;
// Sound controller: Channel on/off
const REG_NR52_ADDR: Addr = 0xFF26;

pub const APU_ADDR_LOW_BOUND: Addr = 0xFF10;
pub const APU_ADDR_HIGH_BOUND_INCLUDED: u16 = 0xFF3F;

pub struct APU {}

impl Reset for APU {
    fn reset(&mut self) {
        todo!()
    }
}

impl BusDevice for APU {
    fn read(&self, addr: Addr) -> Word {
        todo!()
    }

    fn write(&mut self, addr: Addr, data: Word) {
        todo!()
    }
}
