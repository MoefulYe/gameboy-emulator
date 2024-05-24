use crate::types::Addr;
use std::collections::HashMap;

pub type Break = bool;
pub const BREAK: Break = true;
pub const NO_BREAK: Break = false;

pub struct BreakPointState {
    enable: bool,
    onread: bool,
    onwrite: bool,
}

impl BreakPointState {
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

pub struct BreakPoints {
    /// 内存断点
    mem: HashMap<Addr, BreakPointState>,
    /// 中断断点
    break_joypad: Break,
    break_serial: Break,
    break_timer: Break,
    break_lcd_stat: Break,
    break_vblank: Break,
}

impl BreakPoints {
    pub fn new() -> Self {
        Self {
            mem: HashMap::new(),
            break_joypad: false,
            break_serial: false,
            break_timer: false,
            break_lcd_stat: false,
            break_vblank: false,
        }
    }

    pub fn break_memread(&self, addr: Addr) -> Break {
        self.mem.get(&addr).map_or(false, |bp| bp.onread())
    }

    pub fn break_memwrite(&self, addr: Addr) -> Break {
        self.mem.get(&addr).map_or(false, |bp| bp.onwrite())
    }

    pub fn break_joypad(&self) -> Break {
        self.break_joypad
    }

    pub fn break_serial(&self) -> Break {
        self.break_serial
    }

    pub fn break_timer(&self) -> Break {
        self.break_timer
    }

    pub fn break_lcd_stat(&self) -> Break {
        self.break_lcd_stat
    }

    pub fn break_vblank(&self) -> Break {
        self.break_vblank
    }
}
