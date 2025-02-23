use std::io::Cursor;

use crate::{
    dev::{Bus, Cart, LoadCartResult, Reset, CPU},
    dump::CPUStateDump,
    error::{EmuResult, EmulatorError, RunWhenAborting},
    log,
    types::ClockCycle,
};
use ::log::error;
// use brotli::{enc::BrotliEncoderParams, CompressorReader, CompressorWriter};
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use tsify_derive::EmulatorUpdateInput;
use wasm_bindgen::prelude::*;
use web_sys::OffscreenCanvasRenderingContext2d;

#[wasm_bindgen(js_name = WasmEmulator)]
#[derive(Serialize, Deserialize)]
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
        pub timestamp: f64,
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
    pub fn update(
        &mut self,
        EmulatorUpdateInput {
            btns,
            cycles,
            timestamp,
        }: EmulatorUpdateInput,
    ) -> EmulatorUpdateResult {
        if let Some(cart) = &mut self.bus.cart {
            cart.update_rtc(timestamp as _)
        }
        self.bus.btns.update(btns);
        let err = self._update(cycles);
        let cpu = self.cpu.dump(&mut self.bus);
        let cycles = self.cycles;
        self.bus.ppu.update_tiles();
        self.bus.ppu.update_screen();
        EmulatorUpdateResult { cycles, cpu, err }
    }

    #[wasm_bindgen(js_name = loadCart)]
    pub fn load_cart(&mut self, rom: Box<[u8]>, timestamp: f64) -> LoadCartResult {
        if self.bus.cart.is_some() {
            self.reset()
        }
        self.bus.load_cart(rom, timestamp as _).into()
    }

    #[wasm_bindgen(js_name = save)]
    pub fn save(&self) -> Option<Box<[u8]>> {
        let mut output = Vec::new();
        // let writer = CompressorWriter::new(&mut output, 4096, 9, 21);
        if let Err(err) = bincode::serialize_into(&mut output, self) {
            error!("{err}");
            None
        } else {
            Some(output.into_boxed_slice())
        }
    }

    #[wasm_bindgen(js_name = load)]
    pub fn load(&mut self, save: Box<[u8]>) -> bool {
        let tile_canvas = self.bus.ppu.tiles_canvas.take();
        let screen_canvas = self.bus.ppu.screen_canvas.take();
        let cursor = Cursor::new(save);
        // let reader = CompressorReader::new(cursor, 4096, 9, 21);
        match bincode::deserialize_from(cursor) {
            Ok(emu) => {
                *self = emu;
                self.bus.ppu.tiles_canvas = tile_canvas;
                self.bus.ppu.screen_canvas = screen_canvas;
                true
            }
            Err(err) => {
                error!("{err}");
                false
            }
        }
    }

    #[wasm_bindgen(js_name = reset)]
    pub fn reset(&mut self) {
        self.cycles = 0;
        self.aborted = false;
        self.cpu.reset();
        self.bus.reset();
    }

    #[wasm_bindgen(js_name = setScreenCanvas)]
    pub fn set_screen_canvas(&mut self, canvas: OffscreenCanvasRenderingContext2d) {
        self.bus.ppu.set_screen_canvas(canvas)
    }

    #[wasm_bindgen(js_name = setTilesCanvas)]
    pub fn set_tiles_canvas(&mut self, canvas: OffscreenCanvasRenderingContext2d) {
        self.bus.ppu.set_tiles_canvas(canvas)
    }

    fn _update(&mut self, cycles: ClockCycle) -> Option<String> {
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
