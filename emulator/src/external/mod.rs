use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = self, js_name = emulatorLogCallback)]
    pub fn emulator_log_callback(level: u8, msg: &str);
    #[wasm_bindgen(js_namespace = self, js_name = emulatorSerialCallback)]
    pub fn emulator_serial_callback(byte: u8);
    #[wasm_bindgen(js_namespace = self, js_name = emulatorAudioCallback)]
    pub fn emulator_audio_callback(left: js_sys::Float32Array, right: js_sys::Float32Array);
}
