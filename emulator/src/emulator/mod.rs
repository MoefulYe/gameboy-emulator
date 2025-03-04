use std::io::Cursor;

use crate::{
    dev::{Bus, LoadCartResult, Reset, CPU},
    dump::CPUStateDump,
    error::{EmuResult, EmulatorError, RunWhenAborting},
    external::emulator_audio_callback,
    output::{
        audio::WebAudioOutput,
        log::{init_logger, log_flush},
        screen::{WebScreenOutput, WebTileOutput},
        serial::{SerialOutput, WebSerialOutput},
    },
    types::ClockCycle,
};
use ::log::error;
// use brotli::{enc::BrotliEncoderParams, CompressorReader, CompressorWriter};
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use tsify_derive::{EmulatorStepInput, EmulatorUpdateInput};
use wasm_bindgen::prelude::*;
use web_sys::OffscreenCanvasRenderingContext2d;

pub const BASE_CLOCK: u32 = 4_194_304;
pub const VISUAL_FREQ_HZ: f64 = 59.7;

#[wasm_bindgen(js_name = WasmEmulator)]
pub struct Emulator {
    core: Core,
    freq_scale: f64,
    serial_output: WebSerialOutput,
    screen_output: WebScreenOutput,
    tile_output: WebTileOutput,
    audio_output: WebAudioOutput,
}

#[derive(Serialize, Deserialize)]
pub struct Core {
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
        pub timestamp: f64,
    }

    #[derive(Deserialize, Tsify)]
    #[tsify(from_wasm_abi)]
    pub struct EmulatorStepInput {
        pub btns: u8,
        pub timestamp: f64,
    }
}

pub use tsify_derive::EmulatorUpdateResult;

#[wasm_bindgen(js_class = WasmEmulator)]
impl Emulator {
    #[wasm_bindgen(constructor)]
    pub fn new(freq_scale: f64, volume: f32) -> Emulator {
        Self {
            core: Core {
                cpu: CPU::new(),
                bus: Bus::new(),
                aborted: false,
                cycles: 0,
            },
            serial_output: WebSerialOutput::new(),
            screen_output: WebScreenOutput::new(),
            tile_output: WebTileOutput::new(),
            audio_output: WebAudioOutput::new(volume),
            freq_scale,
        }
    }
    #[wasm_bindgen(js_name = initLogger)]
    pub fn init_logger() {
        init_logger();
    }

    #[wasm_bindgen(js_name = update)]
    pub fn update(
        &mut self,
        EmulatorUpdateInput { btns, timestamp }: EmulatorUpdateInput,
    ) -> EmulatorUpdateResult {
        if let Some(cart) = &mut self.core.bus.cart {
            cart.update_rtc(timestamp as _)
        }
        self.core.bus.btns.update(btns);
        let cycles = ((BASE_CLOCK as f64) * self.freq_scale / VISUAL_FREQ_HZ) as u32;
        let err = self._update(cycles);
        let cpu = self.core.cpu.dump(&mut self.core.bus);
        let cycles = self.core.cycles;
        self.core.bus.ppu.update_tiles(&mut self.tile_output);
        self.core.bus.ppu.update_screen(&mut self.screen_output);
        if self.freq_scale == 1.0 {
            self.audio_output.update();
        }
        self.audio_output.clear_buffer();
        self.serial_output.flush();
        log_flush();
        EmulatorUpdateResult { cycles, cpu, err }
    }

    #[wasm_bindgen(js_name = step)]
    pub fn step(
        &mut self,
        EmulatorStepInput { btns, timestamp }: EmulatorStepInput,
    ) -> EmulatorUpdateResult {
        if let Some(cart) = &mut self.core.bus.cart {
            cart.update_rtc(timestamp as _)
        }
        self.core.bus.btns.update(btns);
        let err = self._update(1);
        let cpu = self.core.cpu.dump(&mut self.core.bus);
        let cycles = self.core.cycles;
        self.core.bus.ppu.update_tiles(&mut self.tile_output);
        self.core.bus.ppu.update_screen(&mut self.screen_output);
        self.audio_output.clear_buffer();
        self.serial_output.flush();
        log_flush();
        EmulatorUpdateResult { cycles, cpu, err }
    }

    #[wasm_bindgen(js_name = loadCart)]
    pub fn load_cart(&mut self, rom: Box<[u8]>, timestamp: f64) -> LoadCartResult {
        if self.core.bus.cart.is_some() {
            self.reset()
        }
        self.core.bus.load_cart(rom, timestamp as _).into()
    }

    #[wasm_bindgen(js_name = save)]
    pub fn save(&self) -> Option<Box<[u8]>> {
        let mut output = Vec::new();
        // let writer = CompressorWriter::new(&mut output, 4096, 9, 21);
        if let Err(err) = bincode::serialize_into(&mut output, &self.core) {
            error!("{err}");
            None
        } else {
            Some(output.into_boxed_slice())
        }
    }

    #[wasm_bindgen(js_name = load)]
    pub fn load(&mut self, save: Box<[u8]>) -> bool {
        let cursor = Cursor::new(save);
        // let reader = CompressorReader::new(cursor, 4096, 9, 21);
        match bincode::deserialize_from(cursor) {
            Ok(state) => {
                self.core = state;
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
        self.core.cycles = 0;
        self.core.aborted = false;
        self.core.cpu.reset();
        self.core.bus.reset();
        self.audio_output.reset();
    }

    #[wasm_bindgen(js_name = setScreenCanvas)]
    pub fn set_screen_canvas(&mut self, canvas: OffscreenCanvasRenderingContext2d) {
        self.screen_output.set_canvas(canvas);
    }

    #[wasm_bindgen(js_name = setTilesCanvas)]
    pub fn set_tiles_canvas(&mut self, canvas: OffscreenCanvasRenderingContext2d) {
        self.tile_output.set_canvas(canvas);
    }

    #[wasm_bindgen(js_name = setVolume)]
    pub fn set_volume(&mut self, volume: f32) {
        self.audio_output.set_volume(volume);
    }

    #[wasm_bindgen(js_name = setFreqScale)]
    pub fn set_freq_scale(&mut self, freq_scale: f64) {
        self.freq_scale = freq_scale;
    }

    fn _update(&mut self, cycles: ClockCycle) -> Option<String> {
        if self.core.aborted {
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
        self.core.aborted = true;
        Some(err.as_ref().msg())
    }

    fn tick_devices(&mut self, cycles: ClockCycle) -> EmuResult {
        for _ in 0..cycles {
            self.core.bus.apu.tick(&mut self.audio_output);
            let irq0 = self.core.bus.timer.tick();
            let irq1 = self.core.bus.serial.tick(&mut self.serial_output);
            self.core.bus.tick_dma()?;
            let irq2 = self.core.bus.ppu.tick();
            let irq = irq0 | irq1 | irq2;
            self.core.bus.int_flag_reg.add(irq);
        }
        Ok(())
    }

    fn tick(&mut self) -> EmuResult<ClockCycle> {
        let cycles = self.core.cpu.tick(&mut self.core.bus)?;
        self.tick_devices(cycles)?;
        self.core.cycles += cycles;
        Ok(cycles)
    }
}
