use crate::types::{Addr, OpCode};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum EmulatorError {
    #[error("invalid logo {actual:#?}, expected {expected:#?}")]
    InvalidLogo {
        expected: &'static [u8; 48],
        actual: [u8; 48],
    },
    /// 未插入卡带
    #[error("no cartridge")]
    NoCartridge,
    /// 不合法的指令
    #[error("illegal instruction 0x{opcode:02X} at address 0x{addr:04X}")]
    IllegalInstruction { opcode: OpCode, addr: Addr },
    #[error("stop instruction 0x10 at address 0x{addr:04X}")]
    StopInstruction { addr: Addr },
    #[error("run when aborting")]
    RunWhenAborting,
    #[error("invalid checksum: expected 0x{expected:02X}, found 0x{actual:02X}")]
    InvalidChecksum { expected: u8, actual: u8 },
}

impl EmulatorError {
    fn brief(&self) -> &'static str {
        match self {
            EmulatorError::InvalidLogo { .. } => "invalid logo! ERROR CODE: 0X00",
            EmulatorError::NoCartridge => "no cartridge! ERROR CODE: 0x01",
            EmulatorError::IllegalInstruction { .. } => "illegal instruction! ERROR CODE: 0X02",
            EmulatorError::StopInstruction { .. } => "stop instruction! ERROR CODE: 0x03",
            EmulatorError::RunWhenAborting => "run when aborting! ERORR CODE: 0x04",
            EmulatorError::InvalidChecksum { .. } => "invalid checksum! ERROR CODE: 0x05",
        }
    }

    pub fn info(&self) -> Box<EmulatorErrorInfo> {
        let brief = self.brief();
        let msg = self.to_string().into_boxed_str();
        Box::new(EmulatorErrorInfo { brief, msg })
    }
}

pub type BoxedEmulatorError = Box<EmulatorError>;
pub type BoxedEmulatorErrorInfo = Box<EmulatorErrorInfo>;
pub type Result<T = (), E = BoxedEmulatorError> = std::result::Result<T, E>;
pub use ignore_upper_camel_case_func::{EmulatorErrorInfo, Err};

#[allow(non_snake_case)]
mod ignore_upper_camel_case_func {
    use super::*;
    use tsify::Tsify;
    use wasm_bindgen::prelude::*;
    #[inline]
    pub fn Err<T>(e: EmulatorError) -> Result<T> {
        Result::Err(Box::new(e))
    }

    #[derive(Debug, Serialize, Tsify)]
    pub struct EmulatorErrorInfo {
        pub brief: &'static str,
        pub msg: Box<str>,
    }
}
