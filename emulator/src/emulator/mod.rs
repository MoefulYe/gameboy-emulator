use crate::{
    dev::{Bus, PluginCartResult, Reset, CPU},
    dump::CPUStateDump,
    error::{EmuResult, EmulatorError, RunWhenAborting},
    log,
    types::ClockCycle,
};
use serde::Serialize;
use tsify::Tsify;
use tsify_derive::EmulatorUpdateInput;
use wasm_bindgen::prelude::*;
use web_sys::OffscreenCanvasRenderingContext2d;

#[wasm_bindgen(js_name = WasmEmulator)]
pub struct Emulator {
    cpu: CPU,
    bus: Bus,
    aborted: bool,
    cycles: ClockCycle,
}

// Function `__wbg_instanceof_JsType_24d65669860e1289` should have snake_case name, e.g. `__wbg_instanceof_js_type_24d65669860e1289`
#[allow(non_snake_case)]
mod tsify_derive {
    use serde::Deserialize;

    use super::*;
    #[derive(Serialize, Tsify)]
    #[tsify(into_wasm_abi)]
    pub struct EmulatorUpdateResult {
        pub cycles: ClockCycle,
        pub cpu: CPUStateDump,
        pub err: Option<String>,
    }

    #[derive(Deserialize, Tsify)]
    #[tsify(from_wasm_abi)]
    pub struct EmulatorUpdateInput {
        pub btns: u8,
        pub cycles: ClockCycle,
    }
}

pub use tsify_derive::EmulatorUpdateResult;

#[wasm_bindgen(js_class = WasmEmulator)]
impl Emulator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Emulator {
        Self {
            cpu: CPU::new(),
            bus: Bus::new(),
            aborted: false,
            cycles: 0,
        }
    }

    #[wasm_bindgen(js_name = initLogger)]
    pub fn init_logger() {
        log::init_logger();
    }

    #[wasm_bindgen(js_name = update)]
    pub fn update(&mut self, input: EmulatorUpdateInput) -> EmulatorUpdateResult {
        self.bus.btns.update(input.btns);
        let err = self._update(input.cycles);
        let cpu = self.cpu.dump(&self.bus);
        let cycles = self.cycles;
        self.bus.ppu.update_tiles();
        self.bus.ppu.update_screen();
        EmulatorUpdateResult { cycles, cpu, err }
    }
    pub fn _update(&mut self, cycles: ClockCycle) -> Option<String> {
        if self.aborted {
            return self.handle_err(RunWhenAborting);
        }
        let mut clocks = 0;
        while clocks < cycles {
            let res = self.tick();
            match res {
                EmuResult::Ok(cycles) => {
                    clocks += cycles;
                }
                EmuResult::Err(err) => {
                    return self.handle_err(err);
                }
            }
        }
        None
    }

    #[wasm_bindgen(js_name = pluginCart)]
    pub fn plugin_cart(&mut self, cart: Box<[u8]>) -> PluginCartResult {
        self.bus.plugin_cart(cart)
    }
    #[wasm_bindgen(js_name = reset)]
    pub fn reset(&mut self) {
        self.cycles = 0;
        self.aborted = false;
        self.cpu.reset();
        self.bus.reset();
    }

    #[wasm_bindgen(js_name = setCanvas)]
    pub fn set_canvas(&mut self, canvas: OffscreenCanvasRenderingContext2d) {
        self.bus.ppu.set_canvas(canvas)
    }

    #[wasm_bindgen(js_name = setTilesCanvas)]
    pub fn set_tiles_canvas(&mut self, canvas: OffscreenCanvasRenderingContext2d) {
        self.bus.ppu.set_tiles_canvas(canvas)
    }

    #[wasm_bindgen(js_name = plugoutCart)]
    pub fn plugout_cart(&mut self) {
        self.bus.plugout_cart()
    }

    #[wasm_bindgen(js_name = setButtons)]
    pub fn set_buttons(&mut self, btns: u8) {}

    fn handle_err(&mut self, err: impl AsRef<EmulatorError>) -> Option<String> {
        self.aborted = true;
        Some(err.as_ref().msg())
    }

    fn tick_devices(&mut self, cycles: ClockCycle) -> EmuResult {
        for _ in 0..cycles {
            self.bus.tick()?;
        }
        Ok(())
    }

    fn tick(&mut self) -> EmuResult<ClockCycle> {
        let cycles = self.cpu.tick(&mut self.bus)?;
        self.tick_devices(cycles)?;
        self.cycles += cycles;
        Ok(cycles)
    }
}
