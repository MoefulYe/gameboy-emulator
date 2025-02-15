use crate::types::{Addr, OpCode};
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
    pub fn msg(&self) -> String {
        self.to_string()
    }

    pub fn is_stop(&self) -> bool {
        match self {
            EmulatorError::StopInstruction { .. } => true,
            _ => false,
        }
    }
}

impl AsRef<EmulatorError> for EmulatorError {
    fn as_ref(&self) -> &EmulatorError {
        self
    }
}

pub type BoxedEmulatorError = Box<EmulatorError>;
pub type EmuResult<T = (), E = BoxedEmulatorError> = std::result::Result<T, E>;
pub use ignore_upper_camel_case_func::EmuErr;

#[allow(non_snake_case)]
mod ignore_upper_camel_case_func {
    use super::*;
    #[inline]
    pub fn EmuErr<T>(e: EmulatorError) -> EmuResult<T> {
        EmuResult::Err(Box::new(e))
    }
}
