use crate::types::ClockCycle;

pub struct Clock {
    /// 计划运行的时钟周期数
    planned_cycles: u64,
    /// 已经运行的时钟周期数
    actutal_cycles: u64,
}

impl Clock {
    pub fn new() -> Self {
        Self {
            planned_cycles: 0,
            actutal_cycles: 0,
        }
    }

    pub fn reset(&mut self) {
        self.planned_cycles = 0;
        self.actutal_cycles = 0;
    }

    pub fn ticks(&mut self, clocks: ClockCycle) -> ClockCycle {
        self.planned_cycles += clocks as u64;
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
