pub mod cartridge;
pub mod cpu;
pub mod dev;
pub mod emulator;
pub mod error;
pub mod external;
pub mod types;
pub mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, gameboy!");
}
