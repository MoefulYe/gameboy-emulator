use std::fmt::Display;

pub enum EmulatorError {
    InvalidLogo,
    InvalidRamSize,
    InvalidDestCode,
    /// 卡带校验和不正确
    InvalidChecksum,
}

impl Display for EmulatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub type Result<T = (), E = EmulatorError> = std::result::Result<T, E>;
