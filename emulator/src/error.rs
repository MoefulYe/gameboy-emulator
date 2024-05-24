use crate::types::OpCode;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum EmulatorError {
    #[error("invalid logo")]
    InvalidLogo,
    /// 未插入卡带
    #[error("no cartridge")]
    NoCartridge,
    /// 不合法的指令
    #[error("illegal instruction `{opcode}`")]
    IllegalInstruction { opcode: OpCode },
    #[error("stop instruction")]
    StopInstruction,
    #[error("run when aborting")]
    RunWhenAborting,
}

impl EmulatorError {
    fn code(&self) -> u32 {
        use EmulatorError::*;
        match self {
            InvalidLogo => 0,
            NoCartridge => 1,
            IllegalInstruction { .. } => 2,
            StopInstruction => 3,
            RunWhenAborting => 4,
        }
    }

    fn brief(&self) -> &'static str {
        match self {
            EmulatorError::InvalidLogo => "invalid logo",
            EmulatorError::NoCartridge => "no cartridge",
            EmulatorError::IllegalInstruction { .. } => "illegal instruction",
            EmulatorError::StopInstruction => "stop instruction",
            EmulatorError::RunWhenAborting => "run when aborting",
        }
    }

    pub fn info(&self) -> Box<EmulatorErrorInfo> {
        let code = self.code();
        let brief = self.brief();
        let msg = self.to_string().into_boxed_str();
        Box::new(EmulatorErrorInfo { code, brief, msg })
    }
}

#[derive(Debug, Serialize)]
pub struct EmulatorErrorInfo {
    code: u32,
    brief: &'static str,
    msg: Box<str>,
}

pub type BoxedEmulatorError = Box<EmulatorError>;
pub type BoxedEmulatorErrorInfo = Box<EmulatorErrorInfo>;
pub type Result<T = (), E = BoxedEmulatorError> = std::result::Result<T, E>;
