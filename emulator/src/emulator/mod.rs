use crate::{
    cpu::CPU,
    dev::{bus::Bus, clock::Clock},
    error::{EmulatorError, Result},
    types::ClockCycle,
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
    paused: bool,
}

impl Emulator {
    pub fn new(cpu: CPU, bus: Bus) -> Self {
        Self {
            cpu,
            bus,
            clock: Clock::new(),
            paused: false,
        }
    }

    fn clock_devices(&mut self, cycles: ClockCycle, cycles_n: ClockCycle) -> Result {
        todo!()
    }

    fn clock(&mut self) -> Result<ClockCycle> {
        let cycles = self.cpu.clock(&mut self.bus)?;
        //TODO GameBoySpeed
        let cycles_n = cycles;
        self.clock_devices(cycles, cycles_n)?;
        Ok(cycles)
    }

    pub fn update(&mut self, delta_time: f64) -> Result {
        if self.paused {
            return Ok(());
        }
        let ticks = self.clock.ticks(delta_time);
        let mut clocks = 0;
        while clocks < ticks {
            clocks += self.clock()?;
        }
        self.clock.add_cycles(clocks);
        Ok(())
    }

    pub fn handle_err(&mut self, err: EmulatorError) {
        self.paused = true;
        // match err {
        //
        // }
    }

    pub fn reset(&mut self) {
        todo!()
    }

    pub fn bus(&self) -> &Bus {
        &self.bus
    }

    pub fn bus_mut(&mut self) -> &mut Bus {
        &mut self.bus
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn resume(&mut self) {
        self.paused = false;
    }
}
