use serde::{Deserialize, Serialize};

use crate::{
    types::{Addr, Word},
    utils::bits::BitMap,
};

#[derive(Serialize, Deserialize)]
enum BtnCtl {
    UseDirection,
    UseFunction,
}

pub const BUTTON_ADDR: Addr = 0xFF00;
/// 7 6 5 4 3 2 1 0
/// - - 1 0 u d l r
/// - - 0 1 s s b a
#[derive(Serialize, Deserialize)]
pub struct Buttons {
    btns: Word,
    ctl: BtnCtl,
}

impl Default for Buttons {
    fn default() -> Self {
        Self {
            btns: 0xff,
            ctl: BtnCtl::UseDirection,
        }
    }
}

impl Buttons {
    pub fn update(&mut self, btns: Word) {
        self.btns = !btns;
    }

    pub fn read(&self) -> Word {
        match self.ctl {
            BtnCtl::UseDirection => (self.btns & 0b0000_1111) | 0b0010_0000,
            BtnCtl::UseFunction => ((self.btns & 0b1111_0000) >> 4) | 0b0001_0000,
        }
    }

    pub fn write(&mut self, data: Word) {
        if data.test(5) {
            self.ctl = BtnCtl::UseDirection
        } else {
            self.ctl = BtnCtl::UseFunction
        }
    }
}
