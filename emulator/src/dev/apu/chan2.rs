use super::modulation::*;
use super::*;
use crate::dev::MemoryRegion;
use crate::utils::bits::BitMap;

//
// Default register values
//
const DEFAULT_REG_DMG_NR21: Word = 0x3F;
const DEFAULT_REG_DMG_NR22: Word = 0x00;
const DEFAULT_REG_DMG_NR23: Word = 0xFF;
const DEFAULT_REG_DMG_NR24: Word = 0xBF;

#[derive(Serialize, Deserialize, Debug)]
pub struct Chan2 {
    enabled: bool,
    /// Bit 7-6: Wave Pattern Duty
    /// Bit 5-0: Sound length
    reg_nr21: Word,
    /// Bit 7-4: Initial Volume of envelope (0-0Fh) (0=No Sound)
    /// Bit 3  : Envelope Direction (0=Decrease, 1=Increase)
    /// Bit 2-0: Number of envelope sweep (n: 0-7)
    reg_nr22: Word,
    /// Frequency lower 8 bits
    reg_nr23: Word,
    /// Bit 7  : Initial (1=Restart Sound)
    /// Bit 6  : Counter/consecutive selection (Read/Write)
    /// Bit 2-0: Frequency's higher 3 bits (x) (Write Only)
    reg_nr24: Word,
    /// Volume between 0x0 and 0xF
    current_volume: Word,
    /// Envelope Period timer
    envelope_timer: Word,
    /// Wave cursor position
    wave_cursor: Word,
    /// Frequency timer between 4 and 8192
    frequency_timer: u16,
    /// Length counter between 0 and 64
    length_counter: Word,
    /// Length period is half
    length_half_period: bool,
}

impl Chan2 {
    pub fn new() -> Self {
        Self {
            enabled: false,
            reg_nr21: DEFAULT_REG_DMG_NR21,
            reg_nr22: DEFAULT_REG_DMG_NR22,
            reg_nr23: DEFAULT_REG_DMG_NR23,
            reg_nr24: DEFAULT_REG_DMG_NR24,
            current_volume: DEFAULT_REG_DMG_NR22 >> 4,
            envelope_timer: DEFAULT_REG_DMG_NR22 & 0b111,
            wave_cursor: 0,
            frequency_timer: 4,
            length_counter: 64,
            length_half_period: false,
        }
    }
}

impl Channel for Chan2 {
    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn is_dac_enabled(&self) -> bool {
        (self.reg_nr22 & 0b1111_1000) != 0
    }

    fn trigger(&mut self) {
        if self.is_dac_enabled() {
            self.enabled = true;
        }
        if self.length_counter == 0 {
            self.set_length_counter(64);
            if self.is_length_enabled() && self.length_half_period {
                self.length_counter -= 1;
            }
        }
        self.reset_frequency_timer();
        self.reset_envelope();
        self.reset_wave();
    }
}

impl Clock for Chan2 {
    fn frequency(&self) -> u32 {
        (((self.reg_nr24 & 0b0000_0111) as u32) << 8) | (self.reg_nr23 as u32)
    }

    fn frequency_timer(&self) -> u32 {
        self.frequency_timer as u32
    }

    fn set_frequency_timer(&mut self, value: u32) {
        self.frequency_timer = value as u16;
    }
}

impl EnvelopeModulation for Chan2 {
    fn envelope_register(&self) -> Word {
        self.reg_nr22
    }

    fn envelope_timer(&mut self) -> &mut Word {
        &mut self.envelope_timer
    }

    fn envelope_volume(&self) -> Word {
        self.current_volume
    }

    fn set_envelope_volume(&mut self, value: Word) {
        self.current_volume = value;
    }
}

impl LengthModulation for Chan2 {
    fn is_length_enabled(&self) -> bool {
        //is_set!(self.reg_nr24, 0b0100_0000)
        self.reg_nr24.test(6)
    }

    fn length_counter(&self) -> u16 {
        self.length_counter as u16
    }

    fn set_length_counter(&mut self, value: u16) {
        self.length_counter = value as Word;
    }

    fn set_half_length_period(&mut self, half: bool) {
        self.length_half_period = half;
    }
}

impl WaveModulation for Chan2 {
    fn wave_cursor(&self) -> Word {
        self.wave_cursor
    }

    fn set_wave_cursor(&mut self, value: Word) {
        self.wave_cursor = value;
    }

    fn wave_duty(&self) -> Word {
        self.reg_nr21 >> 6
    }
}

impl MemoryRegion for Chan2 {
    fn read(&self, address: Addr) -> Word {
        match address {
            REG_NR21_ADDR => self.reg_nr21 | 0b0011_1111,
            REG_NR22_ADDR => self.reg_nr22,
            REG_NR23_ADDR => 0xFF,
            REG_NR24_ADDR => self.reg_nr24 | 0b1011_1111,
            _ => unreachable!(),
        }
    }

    fn write(&mut self, address: Addr, value: Word) {
        match address {
            REG_NR21_ADDR => {
                self.length_counter = 64 - (value & 0b0011_1111);
                self.reg_nr21 = value
            }
            REG_NR22_ADDR => {
                self.reg_nr22 = value;
                if !self.is_dac_enabled() {
                    self.enabled = false;
                }
            }
            REG_NR23_ADDR => self.reg_nr23 = value,
            REG_NR24_ADDR => {
                //let trigger = is_set!(value, 0b1000_0000);
                //let length_enabled = is_set!(value, 0b0100_0000);
                let trigger = value.test(7);
                let length_enabled = value.test(6);

                if self.length_half_period
                    && !self.is_length_enabled()
                    && length_enabled
                    && self.length_counter > 0
                {
                    self.length_counter -= 1;
                    if self.length_counter == 0 {
                        self.enabled = false;
                    }
                }
                self.reg_nr24 = value;
                // trigger a channel restart
                if trigger {
                    self.trigger();
                }
            }
            _ => unreachable!(),
        }
    }
}
