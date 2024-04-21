use crate::{
    cpu::CPU,
    dev::{bus::Bus, clock::Clock},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameBoySpeed {
    Normal = 0,
    Double = 1,
}

impl GameBoySpeed {
    pub fn description(&self) -> &'static str {
        match self {
            GameBoySpeed::Normal => "Normal Speed",
            GameBoySpeed::Double => "Double Speed",
        }
    }

    pub fn multiplier(&self) -> u8 {
        match self {
            GameBoySpeed::Normal => 1,
            GameBoySpeed::Double => 2,
        }
    }

    pub fn from(val: u8) -> Option<Self> {
        match val {
            0 => Some(Self::Normal),
            1 => Some(Self::Double),
            _ => None,
        }
    }
}

pub struct Emulator {
    cpu: CPU,
    bus: Bus,
    clock: Clock,
}

impl Emulator {
    pub fn new(cpu: CPU, bus: Bus) -> Self {
        Self {
            cpu,
            bus,
            clock: Clock::new(),
        }
    }

    fn clock_devices(&mut self, cycles: u32, cycles_n: u32) {
        todo!()
    }

    fn clock(&mut self) -> u32 {
        let cycles = self.cpu.clock(&mut self.bus);
        //TODO GameBoySpeed
        let cycles_n = cycles;
        self.clock_devices(cycles, cycles_n);
        cycles
    }

    pub fn update(&mut self, delta_time: f64) {
        let ticks = self.clock.ticks(delta_time);
        let mut clocks = 0u32;
        while clocks < ticks {
            clocks += self.clock()
        }
        self.clock.add_cycles(clocks);
        todo!();
    }

    pub fn reset(&mut self) {
        todo!()
    }
}
