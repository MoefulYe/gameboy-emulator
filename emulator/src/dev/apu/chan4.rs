use super::modulation::*;
use super::*;
use crate::dev::MemoryRegion;
use crate::utils::bits::BitMap;

//
// Default register values
//
const DEFAULT_REG_DMG_NR41: Word = 0x3F;
const DEFAULT_REG_DMG_NR42: Word = 0x00;
const DEFAULT_REG_DMG_NR43: Word = 0xFF;
const DEFAULT_REG_DMG_NR44: Word = 0xBF;

#[derive(Serialize, Deserialize, Debug)]
pub struct Chan4 {
    enabled: bool,
    /// Bit 5-0: Sound length
    reg_nr41: Word,
    /// Bit 7-4: Initial Volume of envelope (0-0Fh) (0=No Sound)
    /// Bit 3  : Envelope Direction (0=Decrease, 1=Increase)
    /// Bit 2-0: Number of envelope sweep (n: 0-7)
    reg_nr42: Word,
    /// Polynomial counter
    /// Bit 7-4: Shift Clock Frequency (s)
    /// Bit 3  : Counter Step/Width (0=15 bits, 1=7 bits)
    /// Bit 2-0: Dividing Ratio of Frequencies (r)
    reg_nr43: Word,
    /// Bit 7  : Initial (1=Restart Sound)
    /// Bit 6  : Counter/consecutive selection (Read/Write)
    reg_nr44: Word,
    /// Volume between 0x0 and 0xF
    current_volume: Word,
    /// Envelope Period timer
    envelope_timer: Word,
    /// Frequency timer between 4 and 8192
    frequency_timer: u32,
    /// Length counter between 0 and 64
    length_counter: Word,
    /// Length period is half
    length_half_period: bool,
    /// Linear feedback shift register (15 bits)
    lfsr: u16,
}

impl Chan4 {
    pub fn new() -> Self {
        Self {
            enabled: false,
            reg_nr41: DEFAULT_REG_DMG_NR41,
            reg_nr42: DEFAULT_REG_DMG_NR42,
            reg_nr43: DEFAULT_REG_DMG_NR43,
            reg_nr44: DEFAULT_REG_DMG_NR44,
            current_volume: DEFAULT_REG_DMG_NR42 >> 4,
            envelope_timer: DEFAULT_REG_DMG_NR42 & 0b111,
            frequency_timer: 4,
            length_counter: 64,
            length_half_period: false,
            lfsr: 0,
        }
    }

    #[inline]
    fn shift_clock_frequency(&self) -> Word {
        self.reg_nr43 >> 4
    }

    #[inline]
    fn is_width_mode_set(&self) -> bool {
        //is_set!(self.reg_nr43, 0b0000_1000)
        self.reg_nr43.test(3)
    }

    #[inline]
    fn divisor_code(&self) -> Word {
        self.reg_nr43 & 0b0000_0111
    }
}

impl Channel for Chan4 {
    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn is_dac_enabled(&self) -> bool {
        (self.reg_nr42 & 0b1111_1000) != 0
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
        self.reset_envelope();
        self.lfsr = 0x7fff;
    }
}

impl Clock for Chan4 {
    fn frequency(&self) -> u32 {
        // Divisor code  Divisor
        // -----------------------
        // 0             8
        // 1            16
        // 2            32
        // 3            48
        // 4            64
        // 5            80
        // 6            96
        // 7           112
        let divisor = if self.divisor_code() == 0 {
            8
        } else {
            (self.divisor_code() as u32) << 4
        };
        let shift = self.shift_clock_frequency();

        divisor << shift
    }

    fn frequency_timer(&self) -> u32 {
        self.frequency_timer
    }

    fn set_frequency_timer(&mut self, value: u32) {
        self.frequency_timer = value;
    }

    fn reset_frequency_timer(&mut self) {
        let new_frequency = self.frequency();
        self.set_frequency_timer(new_frequency);
    }
}

impl Tick for Chan4 {
    fn tick(&mut self) {
        if self.frequency_timer == 0 {
            // frequency is 11 bits large = 0x7FF
            self.reset_frequency_timer();
            let x = (self.lfsr & 0b01) ^ ((self.lfsr >> 1) & 0b01);
            self.lfsr = (self.lfsr >> 1) | (x << 14);
            if self.is_width_mode_set() {
                // Set bit 6
                self.lfsr = (self.lfsr & !(1 << 6)) | (x << 6);
            }
        }

        self.frequency_timer -= 1;
    }
}

impl Sample for Chan4 {
    fn sample(&self) -> Word {
        (!self.lfsr & 0x01) as Word
    }
}

impl EnvelopeModulation for Chan4 {
    fn envelope_register(&self) -> Word {
        self.reg_nr42
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

impl LengthModulation for Chan4 {
    fn is_length_enabled(&self) -> bool {
        //is_set!(self.reg_nr44, 0b0100_0000)
        self.reg_nr44.test(6)
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

impl MemoryRegion for Chan4 {
    fn read(&self, address: Addr) -> Word {
        match address {
            REG_NR41_ADDR => 0xFF,
            REG_NR42_ADDR => self.reg_nr42,
            REG_NR43_ADDR => self.reg_nr43,
            REG_NR44_ADDR => self.reg_nr44 | 0b1011_1111,
            _ => unreachable!(),
        }
    }

    fn write(&mut self, address: Addr, value: Word) {
        match address {
            REG_NR41_ADDR => {
                self.length_counter = 64 - (value & 0b0011_1111);
                self.reg_nr41 = value
            }
            REG_NR42_ADDR => {
                self.reg_nr42 = value;
                if !self.is_dac_enabled() {
                    self.enabled = false;
                }
            }
            REG_NR43_ADDR => self.reg_nr43 = value,
            REG_NR44_ADDR => {
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
                self.reg_nr44 = value;
                // trigger a channel restart
                if trigger {
                    self.trigger();
                }
            }
            _ => unreachable!(),
        }
    }
}
