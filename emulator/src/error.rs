use std::fmt::Display;

pub enum EmulatorError {
    InvalidLogo,
    /// 卡带校验和不正确
    InvalidDestCode,
    /// 未插入卡带
    NoCartridge,
    /// 不合法的指令
    IllegalInstruction,
}

impl Display for EmulatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub type Result<T = (), E = EmulatorError> = std::result::Result<T, E>;
