use wasm_bindgen::prelude::*;

pub type Byte = u8;
pub type Word = u8;
pub type DWord = u16;
pub type Addr = DWord;
pub type OpCode = Word;
/// 时钟周期
pub type ClockCycle = u32;
#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type Byte = number;
export type Word = number;
export type DWord = number;
export type Addr = DWord;
export type OpCode = Word;
export type ClockCycle = number;
"#;
