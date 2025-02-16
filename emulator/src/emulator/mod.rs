use crate::{
    dev::{Bus, PluginCartResult, CPU},
    dump::CPUStateDump,
    error::{EmuResult, EmulatorError, RunWhenAborting},
    log,
    types::ClockCycle,
};
use serde::Serialize;
use tsify::Tsify;
use wasm_bindgen::prelude::*;
use web_sys::OffscreenCanvasRenderingContext2d;

#[wasm_bindgen(js_name = WasmEmulator)]
pub struct Emulator {
    cpu: CPU,
    bus: Bus,
    aborted: bool,
}

#[wasm_bindgen(js_class = WasmEmulator)]
impl Emulator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Emulator {
        Self {
            cpu: CPU::new(),
            bus: Bus::new(),
            aborted: false,
        }
    }

    #[wasm_bindgen(js_name = initLogger)]
    pub fn init_logger() {
        log::init_logger();
    }

    #[wasm_bindgen(js_name = step)]
    pub fn step(&mut self) -> EmulatorStepResult {
        use EmulatorStepResult::*;
        if self.aborted {
            let msg = self.handle_err(RunWhenAborting);
            let cpu_state = self.cpu.dump(&mut self.bus);
            return Abort {
                msg,
                cycles: 0,
                cpu: cpu_state,
            };
        }
        match self.tick() {
            EmuResult::Ok(clock) => {
                let cpu_state = self.cpu.dump(&mut self.bus);
                Ok {
                    cycles: clock,
                    cpu: cpu_state,
                }
            }
            EmuResult::Err(err) => {
                let msg = self.handle_err(err);
                let cpu_state = self.cpu.dump(&mut self.bus);
                Abort {
                    msg,
                    cycles: 0,
                    cpu: cpu_state,
                }
            }
        }
    }

    #[wasm_bindgen(js_name = reset)]
    pub fn reset(&mut self) {
        self.cpu.reset();
        self.bus.reset();
        self.aborted = false
    }

    #[wasm_bindgen(js_name = update)]
    pub fn update(&mut self, cycles: ClockCycle) -> EmulatorUpdateResult {
        use EmulatorUpdateResult::*;
        if self.aborted {
            let msg = self.handle_err(RunWhenAborting);
            let cpu_state = self.cpu.dump(&self.bus);
            return Abort {
                msg,
                cycles: 0,
                cpu: cpu_state,
            };
        }
        let mut clocks = 0;
        while clocks < cycles {
            let res = self.tick();
            match res {
                EmuResult::Ok(cycles) => {
                    clocks += cycles;
                }
                EmuResult::Err(err) => {
                    let msg = self.handle_err(err);
                    let cpu_state = self.cpu.dump(&self.bus);
                    return Abort {
                        msg,
                        cycles: clocks,
                        cpu: cpu_state,
                    };
                }
            }
        }
        let cpu_state = self.cpu.dump(&self.bus);
        Ok {
            cycles: clocks,
            cpu: cpu_state,
        }
    }

    #[wasm_bindgen(js_name = pluginCart)]
    pub fn plugin_cart(&mut self, cart: Box<[u8]>) -> PluginCartResult {
        self.bus.plugin_cart(cart)
    }

    #[wasm_bindgen(js_name = setCanvas)]
    pub fn set_canvas(&mut self, canvas: OffscreenCanvasRenderingContext2d) {
        self.bus.set_canvas(canvas)
    }

    #[wasm_bindgen(js_name = plugoutCart)]
    pub fn plugout_cart(&mut self) {
        self.bus.plugout_cart()
    }

    #[wasm_bindgen(js_name = setButtons)]
    pub fn set_buttons(&mut self, btns: u8) {}

    fn handle_err(&mut self, err: impl AsRef<EmulatorError>) -> String {
        self.aborted = true;
        err.as_ref().msg()
    }

    fn tick_devices(&mut self, cycles: ClockCycle) {
        for _ in 0..cycles {
            self.bus.tick();
        }
    }

    fn tick(&mut self) -> EmuResult<ClockCycle> {
        let cycles = self.cpu.tick(&mut self.bus)?;
        self.tick_devices(cycles);
        Ok(cycles)
    }
}

// Function `__wbg_instanceof_JsType_24d65669860e1289` should have snake_case name, e.g. `__wbg_instanceof_js_type_24d65669860e1289`
#[allow(non_snake_case)]
mod tsify_derive {
    use super::*;
    #[derive(Serialize, Tsify)]
    #[tsify(into_wasm_abi)]
    #[serde(tag = "status")]
    pub enum EmulatorUpdateResult {
        #[serde(rename = "ok")]
        Ok {
            cycles: ClockCycle,
            cpu: CPUStateDump,
        },
        #[serde(rename = "abort")]
        Abort {
            cycles: ClockCycle,
            msg: String,
            cpu: CPUStateDump,
        },
    }

    #[derive(Serialize, Tsify)]
    #[tsify(into_wasm_abi)]
    #[serde(tag = "status")]
    pub enum EmulatorStepResult {
        #[serde(rename = "ok")]
        Ok {
            cycles: ClockCycle,
            cpu: CPUStateDump,
        },
        #[serde(rename = "abort")]
        Abort {
            cycles: ClockCycle,
            msg: String,
            cpu: CPUStateDump,
        },
    }
}

pub use tsify_derive::{EmulatorStepResult, EmulatorUpdateResult};
