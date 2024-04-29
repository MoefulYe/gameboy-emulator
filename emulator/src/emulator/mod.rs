use std::fmt::Display;

use log::{info, warn};

use crate::{
    cpu::CPU,
    dev::{bus::Bus, clock::Clock},
    error::{EmulatorError, Result},
    types::ClockCycle,
};
/// 表示模拟器运行状态
/// Runnning -- 点击暂停 --> Paused
/// Paused -- 点击继续 --> Running
/// Running -- 触发异常 --> Stopped
/// Stopped -- 点击重置 --> Running
#[derive(PartialEq, Eq)]
enum State {
    /// 正常运行
    Running,
    /// 用户暂停
    Paused,
    /// 调用了stop指令，或者触发了异常，智能通过reset恢复
    Stopped,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Running => f.write_str("running"),
            State::Paused => f.write_str("paused"),
            State::Stopped => f.write_str("stopped"),
        }
    }
}

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
    state: State,
}

impl Emulator {
    pub fn new(cpu: CPU, bus: Bus) -> Self {
        Self {
            cpu,
            bus,
            clock: Clock::new(),
            state: State::Running,
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
        if self.state != State::Running {
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
        self.state = State::Stopped;
        // match err {}
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.bus.reset();
        self.clock.reset();
        self.state = State::Running;
    }

    pub fn bus(&self) -> &Bus {
        &self.bus
    }

    pub fn bus_mut(&mut self) -> &mut Bus {
        &mut self.bus
    }

    pub fn pause(&mut self) {
        self.state = State::Paused;
    }

    pub fn resume(&mut self) {
        match self.state {
            State::Running => info!("emulator is already running"),
            State::Paused => self.state = State::Running,
            State::Stopped => warn!("emulator has stopped, please reset it first"),
        }
    }
}
