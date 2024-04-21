use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = emulatorLog)]
    pub fn emulator_log(level: u8, message: &str);
}
