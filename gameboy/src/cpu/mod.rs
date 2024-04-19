use self::regs::Regs;

pub mod regs;

pub struct CPU {
    regs: Regs,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            regs: Default::default(),
        }
    }
}
