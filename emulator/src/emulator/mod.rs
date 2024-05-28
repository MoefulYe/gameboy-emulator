use crate::{
    dev::{Bus, Button, CPU, NO_BREAK},
    error::{BoxedEmulatorError, BoxedEmulatorErrorInfo, EmulatorError, EmulatorErrorInfo, Result},
    log,
    trace::CPUState,
    types::ClockCycle,
};
use serde::Serialize;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

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

    pub fn _step(&mut self) -> EmulatorStepResult {
        use EmulatorStepResult::*;
        if self.stopped {
            let info = self.handle_err(Box::new(EmulatorError::RunWhenAborting));
            return Abort { info };
        }
        let pc = self.cpu.pc();
        let res = self.tick();
        match res {
            Result::Ok((clock, _)) => {
                let cpu = self.cpu.trace(&mut self.bus, pc);
                Ok { cycles: clock, cpu }
            }
            Result::Err(err) => {
                let info = self.handle_err(err);
                Abort { info }
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

    pub fn update(&mut self, cycles: ClockCycle) -> EmulatorUpdateResult {
        use EmulatorUpdateResult::*;
        if self.stopped {
            let info = self.handle_err(Box::new(EmulatorError::RunWhenAborting));
            return Abort { info, cycles: 0 };
        }
        let mut clocks = 0;
        while clocks < cycles {
            let pc = self.cpu.pc();
            let res = self.tick();
            match res {
                Result::Ok((cycles, brk)) => {
                    clocks += cycles;
                    if brk {
                        return Break {
                            cycles: clocks,
                            cpu: self.cpu.trace(&mut self.bus, pc),
                        };
                    }
                }
                Result::Err(err) => {
                    let info = self.handle_err(err);
                    return Abort {
                        info,
                        cycles: clocks,
                    };
                }
            }
        }
        Ok { cycles: clocks }
    }

    pub fn plugin_cart(&mut self, cart: Box<[u8]>) -> Option<Box<EmulatorErrorInfo>> {
        self.bus.plugin_cart(cart)
    }

    pub fn plugout_cart(&mut self) {
        self.bus.plugout_cart()
    }

    fn handle_err(&mut self, err: BoxedEmulatorError) -> BoxedEmulatorErrorInfo {
        self.stopped = true;
        err.info()
    }

    fn tick_devices(&mut self, cycles: ClockCycle) -> bool {
        let mut brk = NO_BREAK;
        for _ in 0..cycles {
            brk |= self.bus.tick();
        }
        brk
    }

    fn tick(&mut self) -> Result<(ClockCycle, bool)> {
        let (cycles, brk0) = self.cpu.tick(&mut self.bus)?;
        let brk1 = self.tick_devices(cycles);
        Ok((cycles, brk0 || brk1))
    }
}

// Function `__wbg_instanceof_JsType_24d65669860e1289` should have snake_case name, e.g. `__wbg_instanceof_js_type_24d65669860e1289`
#[allow(non_snake_case)]
mod tsify_derive {
    use crate::error::EmulatorErrorInfo;

    use super::*;
    #[derive(Serialize, Tsify)]
    #[tsify(into_wasm_abi)]
    #[serde(tag = "status")]
    pub enum EmulatorUpdateResult {
        #[serde(rename = "ok")]
        Ok { cycles: ClockCycle },
        #[serde(rename = "break")]
        Break {
            cycles: ClockCycle,
            cpu: Box<CPUState>,
        },
        #[serde(rename = "abort")]
        Abort {
            cycles: ClockCycle,
            info: Box<EmulatorErrorInfo>,
        },
    }

    #[derive(Serialize, Tsify)]
    #[tsify(into_wasm_abi)]
    #[serde(tag = "status")]
    pub enum EmulatorStepResult {
        #[serde(rename = "ok")]
        Ok {
            cycles: ClockCycle,
            cpu: Box<CPUState>,
        },
        #[serde(rename = "abort")]
        Abort { info: Box<EmulatorErrorInfo> },
    }
}

pub use tsify_derive::{EmulatorStepResult, EmulatorUpdateResult};
