use wasm_bindgen::prelude::*;

use crate::{
    dev::{Bus, Button, CPU},
    error::{EmulatorError, Result},
    log,
    types::ClockCycle,
};

#[wasm_bindgen(js_name = WasmEmulator)]
pub struct Emulator {
    cpu: CPU,
    bus: Bus,
    stopped: bool,
}

#[wasm_bindgen(js_class = WasmEmulator)]
impl Emulator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Emulator {
        Self {
            cpu: CPU::new(),
            bus: Bus::new(),
            stopped: false,
        }
    }

    #[wasm_bindgen(js_name = initLogger)]
    pub fn init_logger() {
        log::init_logger();
    }

    pub fn step(&mut self) -> Result<ClockCycle, String> {
        if self.stopped {
            return Ok(0);
        }
        match self.tick() {
            Ok(clock) => Ok(clock),
            Err(err) => {
                let err_code = self.handle_err(err);
                Err(err_code)
            }
        }
    }

    pub fn update(&mut self, cycles: ClockCycle) -> Result<ClockCycle, String> {
        let res = self._update(cycles);
        match res {
            Ok(cycle) => Ok(cycle),
            Err(err) => {
                let err_code = self.handle_err(err);
                Err(err_code)
            }
        }
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.bus.reset();
        self.stopped = false
    }

    pub fn up(&mut self, btn: Button) {
        todo!()
    }

    pub fn down(&mut self, btn: Button) {
        todo!()
    }

    fn _update(&mut self, cycles: ClockCycle) -> Result<ClockCycle> {
        if self.stopped {
            return Ok(0);
        }
        let mut clocks = 0;
        while clocks < cycles {
            clocks += self.tick()?;
        }
        Ok(clocks)
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
