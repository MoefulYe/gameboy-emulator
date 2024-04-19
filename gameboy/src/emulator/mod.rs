use crate::{cpu::CPU, dev::bus::Bus};

pub struct Emulator {
    cpu: CPU,
    bus: Bus,
}

impl Emulator {
    pub fn new(cpu: CPU, bus: Bus) -> Self {
        Self { cpu, bus }
    }

    pub fn step(&mut self) {}
}
