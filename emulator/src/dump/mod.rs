use crate::types::{DWord, Word};
use serde::Serialize;
use tsify::Tsify;

#[allow(non_snake_case)]
mod allow_non_snake_case {
    use super::*;
    #[derive(Serialize, Tsify)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, Copy)]
    pub struct CPUStateDump {
        pub ime: bool,
        pub halted: bool,
        pub a: Word,
        pub f: Word,
        pub b: Word,
        pub c: Word,
        pub d: Word,
        pub e: Word,
        pub h: Word,
        pub l: Word,
        pub af: DWord,
        pub bc: DWord,
        pub de: DWord,
        pub hl: DWord,
        pub pc: DWord,
        pub sp: DWord,
        pub zero_flag: bool,
        pub negative_flag: bool,
        pub half_flag: bool,
        pub carry_flag: bool,
        pub inst: &'static str,
        pub three_words_at_pc: [Word; 3],
    }
}

pub use allow_non_snake_case::CPUStateDump;
