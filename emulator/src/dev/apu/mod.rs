use chan1::Chan1;
use chan2::Chan2;
use chan3::Chan3;
use chan4::Chan4;
use log::debug;
use serde::{Deserialize, Serialize};

use super::{MemoryRegion, Reset};
use crate::emulator::BASE_CLOCK;
use crate::output::audio::AudioOutput;
use crate::types::{Addr, Word};
use crate::utils::bits::BitMap;
use modulation::*;

mod chan1;
mod chan2;
mod chan3;
mod chan4;
mod modulation;

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

const DEFAULT_REG_DMG_NR50: Word = 0x77;
const DEFAULT_REG_DMG_NR51: Word = 0xF3;
const DEFAULT_REG_DMG_NR52: Word = 0xF1;

pub const AUDIO_SAMPLE_RATE: u32 = 48000;
const SAMPLE_PERIOD: u32 = BASE_CLOCK / AUDIO_SAMPLE_RATE;
const FRAME_SEQUENCER_RATE: u32 = 512; // Hz
const FRAME_SEQUENCER_PERIOD: u32 = BASE_CLOCK / FRAME_SEQUENCER_RATE;

#[derive(Serialize, Deserialize)]
pub struct APU {
    /// Channel control / ON-OFF / Volume (R/W)
    /// Bit   7: Output Vin to SO2 terminal (1=Enable)
    /// Bit 6-4: SO2 output level (volume)  (0-7)
    /// Bit   3: Output Vin to SO1 terminal (1=Enable)
    /// Bit 2-0: SO1 output level (volume)  (0-7)
    reg_nr50: Word,
    /// Selection of Sound output terminal (R/W)
    /// Bit   7: Output sound 4 to SO2 terminal
    /// Bit   6: Output sound 3 to SO2 terminal
    /// Bit   5: Output sound 2 to SO2 terminal
    /// Bit   4: Output sound 1 to SO2 terminal
    /// Bit   3: Output sound 4 to SO1 terminal
    /// Bit   2: Output sound 3 to SO1 terminal
    /// Bit   1: Output sound 2 to SO1 terminal
    /// Bit   0: Output sound 1 to SO1 terminal
    reg_nr51: Word,
    /// Sound on/off
    /// Bit   7: All sound on/off  (0: stop all sound circuits) (Read/Write)
    /// Bit   3: Sound 4 ON flag (Read Only)
    /// Bit   2: Sound 3 ON flag (Read Only)
    /// Bit   1: Sound 2 ON flag (Read Only)
    /// Bit   0: Sound 1 ON flag (Read Only)
    reg_nr52: Word,
    /// Number of ticks before stepping up the frame sequencer
    ticks: u32,
    /// Frame sequencer step % 8
    fs_step: Word,
    /// Sound Channel 1 - Tone & Sweep
    chan1: Chan1,
    /// Sound Channel 2 - Tone
    chan2: Chan2,
    /// Sound Channel 3 - Wave Output
    chan3: Chan3,
    /// Sound Channel 4 - Noise
    chan4: Chan4,
}

impl APU {
    pub fn new() -> Self {
        Self {
            reg_nr50: DEFAULT_REG_DMG_NR50,
            reg_nr51: DEFAULT_REG_DMG_NR51,
            reg_nr52: DEFAULT_REG_DMG_NR52,
            ticks: 0,
            fs_step: 0,
            chan1: Chan1::new(),
            chan2: Chan2::new(),
            chan3: Chan3::new(),
            chan4: Chan4::new(),
        }
    }

    #[inline]
    fn is_enabled(&self) -> bool {
        self.reg_nr52.test(7)
    }

    #[inline]
    fn volume_left(&self) -> u8 {
        (self.reg_nr50 >> 4) & 0b0000_0111
    }

    #[inline]
    fn volume_right(&self) -> u8 {
        self.reg_nr50 & 0b0000_0111
    }

    fn handle_fs_step(&mut self) {
        let is_length_period = (self.fs_step % 2) == 0;
        self.chan1.set_half_length_period(is_length_period);
        self.chan2.set_half_length_period(is_length_period);
        self.chan3.set_half_length_period(is_length_period);
        self.chan4.set_half_length_period(is_length_period);

        // Step   Length Ctr  Vol Env     Sweep
        // ---------------------------------------
        // 0      Clock       -           -
        // 1      -           -           -
        // 2      Clock       -           Clock
        // 3      -           -           -
        // 4      Clock       -           -
        // 5      -           -           -
        // 6      Clock       -           Clock
        // 7      -           Clock       -
        // ---------------------------------------
        // Rate   256 Hz      64 Hz       128 Hz
        if is_length_period {
            // handle length
            self.chan1.length_step();
            self.chan2.length_step();
            self.chan3.length_step();
            self.chan4.length_step();
            if self.fs_step == 2 || self.fs_step == 6 {
                // handle sweep
                self.chan1.sweep_step();
            }
        } else if self.fs_step == 7 {
            // handle volume
            self.chan1.volume_step();
            self.chan2.volume_step();
            self.chan4.volume_step();
        }
        self.fs_step = (self.fs_step + 1) % 8;
    }

    fn mix_channels(&mut self, flag_offset: u8, volume: u8) -> f32 {
        // normalize volume
        let volume = (volume as f32) / 7.0;
        let mut sample = 0.0f32;
        if (self.reg_nr51 & flag_offset) != 0 {
            sample += self.chan1.dac_output();
        }
        if (self.reg_nr51 & (flag_offset << 1)) != 0 {
            sample += self.chan2.dac_output();
        }
        if (self.reg_nr51 & (flag_offset << 2)) != 0 {
            sample += self.chan3.dac_output();
        }
        if (self.reg_nr51 & (flag_offset << 3)) != 0 {
            sample += self.chan4.dac_output();
        }

        // if is_set!(self.reg_nr51, flag_offset) {
        //     sample += self.chan1.dac_output();
        // }
        // if is_set!(self.reg_nr51, flag_offset << 1) {
        //     sample += self.chan2.dac_output();
        // }
        // if is_set!(self.reg_nr51, flag_offset << 2) {
        //     sample += self.chan3.dac_output();
        // }
        // if is_set!(self.reg_nr51, flag_offset << 3) {
        //     sample += self.chan4.dac_output();
        // }
        (sample * volume) / 4.0
    }

    pub fn tick(&mut self, speaker: &mut impl AudioOutput) {
        self.ticks = self.ticks.wrapping_add(1);

        self.chan3.wave_just_read = false;

        self.chan1.tick();
        self.chan2.tick();
        self.chan3.tick();
        self.chan4.tick();

        // Every 8192 T-cycles, the frame sequencer is stepped
        if self.ticks % FRAME_SEQUENCER_PERIOD == 0 {
            self.handle_fs_step();
        }

        // Every sample period, we can send the current sample to the speaker
        // It's up to the speaker to store an audio buffer and play it a regular interval
        if self.ticks % SAMPLE_PERIOD == 0 {
            let left_volume = self.volume_left();
            let right_volume = self.volume_right();

            let s02 = self.mix_channels(0x10, left_volume);
            let s01 = self.mix_channels(0x01, right_volume);

            speaker.set_samples(s02, s01);
        }
    }
}

impl Reset for APU {
    fn reset(&mut self) {
        *self = Self::new();
    }
}

impl MemoryRegion for APU {
    fn read(&self, addr: Addr) -> Word {
        match addr {
            REG_NR10_ADDR | REG_NR11_ADDR | REG_NR12_ADDR | REG_NR13_ADDR | REG_NR14_ADDR => {
                self.chan1.read(addr)
            }
            REG_NR21_ADDR | REG_NR22_ADDR | REG_NR23_ADDR | REG_NR24_ADDR => self.chan2.read(addr),
            REG_NR30_ADDR
            | REG_NR31_ADDR
            | REG_NR32_ADDR
            | REG_NR33_ADDR
            | REG_NR34_ADDR
            | WAVE_PATTERN_RAM_START..=WAVE_PATTERN_RAM_END => self.chan3.read(addr),
            REG_NR41_ADDR | REG_NR42_ADDR | REG_NR43_ADDR | REG_NR44_ADDR => self.chan4.read(addr),
            REG_NR50_ADDR => self.reg_nr50,
            REG_NR51_ADDR => self.reg_nr51,
            REG_NR52_ADDR => {
                let mut data = (self.reg_nr52 & 0b1000_0000) | 0b0111_0000;
                data |= self.chan1.is_enabled() as Word;
                data |= (self.chan2.is_enabled() as Word) << 1;
                data |= (self.chan3.is_enabled() as Word) << 2;
                data |= (self.chan4.is_enabled() as Word) << 3;
                data
            }
            _ => 0xFF,
        }
    }

    fn write(&mut self, addr: Addr, data: Word) {
        if !self.is_enabled()
            && !(WAVE_PATTERN_RAM_START..=WAVE_PATTERN_RAM_START).contains(&addr)
            && addr != REG_NR11_ADDR
            && addr != REG_NR21_ADDR
            && addr != REG_NR31_ADDR
            && addr != REG_NR41_ADDR
            && addr != REG_NR52_ADDR
        {
            return;
        }
        match addr {
            REG_NR10_ADDR | REG_NR11_ADDR | REG_NR12_ADDR | REG_NR13_ADDR | REG_NR14_ADDR => {
                self.chan1.write(addr, data)
            }
            REG_NR21_ADDR | REG_NR22_ADDR | REG_NR23_ADDR | REG_NR24_ADDR => {
                self.chan2.write(addr, data)
            }
            REG_NR30_ADDR
            | REG_NR31_ADDR
            | REG_NR32_ADDR
            | REG_NR33_ADDR
            | REG_NR34_ADDR
            | WAVE_PATTERN_RAM_START..=WAVE_PATTERN_RAM_END => self.chan3.write(addr, data),
            REG_NR41_ADDR | REG_NR42_ADDR | REG_NR43_ADDR | REG_NR44_ADDR => {
                self.chan4.write(addr, data)
            }
            REG_NR50_ADDR => self.reg_nr50 = data,
            REG_NR51_ADDR => self.reg_nr51 = data,
            REG_NR52_ADDR => {
                //let enabled = is_set!(data, 0b1000_0000);
                let enabled = data.test(7);
                let len_ch1 = self.chan1.length_counter();
                let len_ch2 = self.chan2.length_counter();
                let len_ch3 = self.chan3.length_counter();
                let len_ch4 = self.chan4.length_counter();

                if enabled && !self.is_enabled() {
                    self.fs_step = 0;
                } else if !enabled && self.is_enabled() {
                    for addr in REG_NR10_ADDR..REG_NR52_ADDR {
                        self.write(addr, 0x00);
                    }
                }
                // restore old counters
                self.chan1.set_length_counter(len_ch1);
                self.chan2.set_length_counter(len_ch2);
                self.chan3.set_length_counter(len_ch3);
                self.chan4.set_length_counter(len_ch4);

                self.reg_nr52 = data & 0x80
            }
            _ => {}
        }
    }
}
