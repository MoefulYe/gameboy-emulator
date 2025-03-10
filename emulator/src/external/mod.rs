use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = self, js_name = emulatorLogCallback)]
    pub fn emulator_log_callback(logs: JsValue);
    #[wasm_bindgen(js_namespace = self, js_name = emulatorSerialCallback)]
    pub fn emulator_serial_callback(byte: &[u8]);
    #[wasm_bindgen(js_namespace = self, js_name = emulatorAudioCallback)]
    pub fn emulator_audio_callback(left_buffer: &[f32], right_buffer: &[f32]);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn console_log(msg: &str);
}
