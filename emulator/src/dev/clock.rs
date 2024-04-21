pub struct Clock {
    freq: f64,
    cycles: u32,
}

impl Clock {
    pub fn new() -> Self {
        Self {
            freq: Self::BASE_FREQ,
            cycles: 0,
        }
    }

    pub fn with_freq(freq: f64) -> Self {
        Self { freq, cycles: 0 }
    }

    pub fn set_freq(&mut self, freq: f64) {
        self.freq = freq
    }

    pub fn reset(&mut self) {
        self.freq = Self::BASE_FREQ
    }

    const BASE_FREQ: f64 = 4_194_304.0;

    pub fn ticks(&self, delta_time: f64) -> u32 {
        (self.freq * delta_time) as u32
    }

    pub fn add_cycles(&mut self, clocks: u32) {
        self.cycles += clocks
    }

    pub fn cycles(&self) -> u32 {
        self.cycles
    }
}
