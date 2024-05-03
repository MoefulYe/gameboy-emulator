use log::warn;

use crate::{
    error::Result,
    types::{Addr, DWord, Word},
    utils::bits::BitMap,
};

use super::{BusDevice, TickResult, Tickable};

const TIMER_DIV_REG_ADDR: Addr = 0xFF04;
const TIMER_TIMA_REG_ADDR: Addr = 0xFF05;
const TIMER_TMA_REG_ADDR: Addr = 0xFF06;
const TIMER_TAC_REG_ADDR: Addr = 0xFF07;

pub const TIMER_ADDR_LOW_BOUND: Addr = TIMER_DIV_REG_ADDR;
pub const TIMER_ADDR_HIGH_BOUND_INCLUDED: Addr = TIMER_TAC_REG_ADDR;
/// ref https://gbdev.io/pandocs/Timer_and_Divider_Registers.html#ff04--div-divider-register
pub struct Timer {
    div: DWord,
    tima: Word,
    tma: Word,
    tac: Word,
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            div: 0xAC00,
            tima: 0x00,
            tma: 0x00,
            tac: 0xF8,
        }
    }
}

impl Timer {
    pub fn new() -> Self {
        Default::default()
    }

    fn read_div(&self) -> Word {
        (self.div >> 8) as Word
    }
    fn disabled(&self) -> bool {
        self.tac & 0x04 == 0
    }

    fn clock_select(&self) -> Word {
        self.tac & 0x03
    }
}

impl BusDevice for Timer {
    fn read(&self, addr: Addr) -> Result<Word> {
        match addr {
            TIMER_DIV_REG_ADDR => Ok(self.read_div()),
            TIMER_TIMA_REG_ADDR => Ok(self.tima),
            TIMER_TMA_REG_ADDR => Ok(self.tma),
            TIMER_TAC_REG_ADDR => Ok(self.tac | 0xF8),
            _ => {
                warn!("illegal read from timer at address: 0x{addr:04X}");
                Ok(0xFF)
            }
        }
    }

    fn write(&mut self, addr: Addr, data: Word) -> Result {
        match addr {
            TIMER_DIV_REG_ADDR => {
                self.div = 0;
                Ok(())
            }
            TIMER_TIMA_REG_ADDR => {
                self.tima = data;
                Ok(())
            }
            TIMER_TMA_REG_ADDR => {
                self.tma = data;
                Ok(())
            }
            TIMER_TAC_REG_ADDR => {
                self.tac = data;
                Ok(())
            }
            _ => {
                warn!("illegal write to timer at address: 0x{addr:04X}");
                Ok(())
            }
        }
    }
}

impl Tickable for Timer {
    fn tick(&mut self) -> TickResult {
        if self.disabled() {
            return TickResult::Ok;
        }
        let old = self.div;
        let new = old.wrapping_add(1);
        // ref https://gbdev.io/pandocs/Timer_and_Divider_Registers.html#ff07--tac-timer-control
        let pos: DWord = match self.clock_select() {
            // 4096 Hz
            0b00 => 9,
            0b01 => 3,
            0b10 => 5,
            0b11 => 7,
            _ => unreachable!(),
        };
        let update_tima = old.at(pos) && !new.at(pos);
        if update_tima {
            if self.tima == Word::MAX {
                self.tima = self.tma;
                TickResult::IntReq
            } else {
                self.tima += 1;
                TickResult::Ok
            }
        } else {
            TickResult::Ok
        }
    }
}
