use wasm_bindgen::prelude::*;

use crate::{
    cpu::CPU,
    dev::{bus::Bus, buttons::Button, clock::Clock},
    error::{EmulatorError, Result},
    log,
    types::ClockCycle,
};

#[wasm_bindgen(js_name = WasmEmulator)]
pub struct Emulator {
    cpu: CPU,
    bus: Bus,
    clock: Clock,
    stopped: bool,
}

#[wasm_bindgen(js_class = WasmEmulator)]
impl Emulator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Emulator {
        Self {
            cpu: CPU::new(),
            bus: Bus::new(),
            clock: Clock::new(),
            stopped: false,
        }
    }

    #[wasm_bindgen(js_name = initLogger)]
    pub fn init_logger() {
        log::init_logger();
    }

    pub fn step(&mut self) -> Result<(), String> {
        if self.stopped {
            return Ok(());
        }
        match self.tick() {
            Ok(clock) => {
                self.clock.step(clock);
                Ok(())
            }
            Err(err) => {
                let err_code = self.handle_err(err);
                Err(err_code)
            }
        }
    }

    pub fn update(&mut self, cycles: ClockCycle) -> Result<(), String> {
        let res = self._update(cycles);
        match res {
            Ok(_) => Ok(()),
            Err(err) => {
                let err_code = self.handle_err(err);
                Err(err_code)
            }
        }
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.bus.reset();
        self.clock.reset();
        self.stopped = false
    }

    pub fn up(&mut self, btn: Button) {
        todo!()
    }

    pub fn down(&mut self, btn: Button) {
        todo!()
    }

    fn _update(&mut self, cycles: ClockCycle) -> Result {
        if self.stopped {
            return Ok(());
        }
        let ticks = self.clock.ticks(cycles);
        let mut clocks = 0;
        while clocks < ticks {
            clocks += self.tick()?;
        }
        self.clock.add_cycles(clocks);
        Ok(())
    }

    fn tick_devices(&mut self, cycles: ClockCycle) {
        for _ in 0..cycles {
            self.bus.tick();
        }
    }

    fn tick(&mut self) -> Result<ClockCycle> {
        let cycles = self.cpu.tick(&mut self.bus)?;
        self.tick_devices(cycles);
        Ok(cycles)
    }

    fn handle_err(&mut self, err: EmulatorError) -> String {
        self.stopped = true;
        err.to_string()
    }
}
