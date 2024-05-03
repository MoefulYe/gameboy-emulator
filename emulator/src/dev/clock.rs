use crate::types::ClockCycle;

pub struct Clock {
    /// 时钟频率 MHZ
    freq: f64,
    /// 计划运行的时钟周期数
    planned_cycles: u64,
    /// 已经运行的时钟周期数
    actutal_cycles: u64,
}

impl Clock {
    pub fn new() -> Self {
        Self {
            freq: Self::BASE_FREQ,
            planned_cycles: 0,
            actutal_cycles: 0,
        }
    }

    pub fn with_freq(freq: f64) -> Self {
        Self {
            freq,
            planned_cycles: 0,
            actutal_cycles: 0,
        }
    }

    pub fn set_freq(&mut self, freq: f64) {
        self.freq = freq
    }

    pub fn reset(&mut self) {
        self.freq = Self::BASE_FREQ;
        self.planned_cycles = 0;
        self.actutal_cycles = 0;
    }

    const BASE_FREQ: f64 = 4_194_304.0;

    pub fn ticks(&mut self, delta_time: f64) -> ClockCycle {
        let ticks = (self.freq * delta_time) as ClockCycle;
        self.planned_cycles += ticks as u64;
        if self.planned_cycles <= self.actutal_cycles {
            0
        } else {
            (self.planned_cycles - self.actutal_cycles) as ClockCycle
        }
    }

    pub fn add_cycles(&mut self, clocks: ClockCycle) {
        self.actutal_cycles += clocks as u64;
    }

    pub fn step(&mut self, clocks: ClockCycle) {
        self.actutal_cycles += clocks as u64;
        self.planned_cycles += clocks as u64;
    }
}
