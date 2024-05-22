use std::collections::BTreeSet;

use crate::types::Addr;

pub struct BreakPoint {
    enable: bool,
    onread: bool,
    onwrite: bool,
}

impl BreakPoint {
    pub fn new(onread: bool, onwrite: bool) -> Self {
        Self {
            enable: true,
            onread,
            onwrite,
        }
    }

    pub fn enable(&mut self) {
        self.enable = true;
    }

    pub fn disable(&mut self) {
        self.enable = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enable
    }

    pub fn enable_onread(&mut self) {
        self.onread = true;
    }

    pub fn disable_onread(&mut self) {
        self.onread = false;
    }

    pub fn enable_onwrite(&mut self) {
        self.onwrite = true;
    }

    pub fn disable_onwrite(&mut self) {
        self.onwrite = false;
    }

    pub fn onread(&self) -> bool {
        self.enable && self.onread
    }

    pub fn onwrite(&self) -> bool {
        self.enable && self.onwrite
    }
}

pub struct BreakPoints(BTreeSet<Addr>);

impl std::ops::DerefMut for BreakPoints {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::ops::Deref for BreakPoints {
    type Target = BTreeSet<Addr>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl BreakPoints {
    pub fn new() -> Self {
        Self(BTreeSet::new())
    }

    pub fn read_break(&self, addr: Addr) -> bool {
        self.contains(&addr)
    }

    pub fn write_break(&self, addr: Addr) -> bool {
        self.contains(&addr)
    }
}
