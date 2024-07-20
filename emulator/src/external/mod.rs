use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = emulatorLogCallback)]
    pub fn emulator_log_callback(level: u8, msg: &str);
    #[wasm_bindgen(js_namespace = window, js_name = emulatorSerialCallback)]
    pub fn emulator_serial_callback(byte: u8);
}
