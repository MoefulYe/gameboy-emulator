use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = EmulatorButton)]
pub enum Button {
    Right = 0,
    Left = 1,
    Up = 2,
    Down = 3,
    A = 4,
    B = 5,
    Start = 6,
    Select = 7,
}