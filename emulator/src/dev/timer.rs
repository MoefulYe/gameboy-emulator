use log::warn;

use crate::{
    types::{Addr, DWord, Word},
    utils::bits::BitMap,
};

use super::{BusDevice, Tick, TickResult};

const TIMER_DIV_REG_ADDR: Addr = 0xFF04;
const TIMER_TIMA_REG_ADDR: Addr = 0xFF05;
const TIMER_TMA_REG_ADDR: Addr = 0xFF06;
const TIMER_TAC_REG_ADDR: Addr = 0xFF07;

pub const TIMER_ADDR_LOW_BOUND: Addr = TIMER_DIV_REG_ADDR;
pub const TIMER_ADDR_HIGH_BOUND_INCLUDED: Addr = TIMER_TAC_REG_ADDR;
/// ref https://gbdev.io/pandocs/Timer_and_Divider_Registers.html#ff04--div-divider-register
pub struct Timer {
    // Divider
    div: DWord,
    // Timer Counter
    tima: Word,
    // Timer Modulo
    tma: Word,
    // Timer Control
    // 2: enable 1-0: select
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

    fn enabled(&self) -> bool {
        self.tac.test(2)
    }
    fn disabled(&self) -> bool {
        !self.enabled()
    }
    fn clock_select(&self) -> Word {
        self.tac & 0x03
    }
}

impl BusDevice for Timer {
    fn read(&self, addr: Addr) -> Word {
        match addr {
            TIMER_DIV_REG_ADDR => self.read_div(),
            TIMER_TIMA_REG_ADDR => self.tima,
            TIMER_TMA_REG_ADDR => self.tma,
            TIMER_TAC_REG_ADDR => self.tac | 0xF8,
            _ => {
                warn!("illegal read from timer at address: 0x{addr:04X}");
                0xFF
            }
        }
    }

    fn write(&mut self, addr: Addr, data: Word) {
        match addr {
            TIMER_DIV_REG_ADDR => self.div = 0,
            TIMER_TIMA_REG_ADDR => self.tima = data,
            TIMER_TMA_REG_ADDR => self.tma = data,
            TIMER_TAC_REG_ADDR => self.tac = data,
            _ => warn!("illegal write to timer at address: 0x{addr:04X}"),
        }
    }
}

impl Tick for Timer {
    fn tick(&mut self) -> TickResult {
        let prev = self.div;
        let cur = prev.wrapping_add(1);
        self.div = cur;
        if self.disabled() {
            return TickResult::Ok;
        }
        // ref https://gbdev.io/pandocs/Timer_and_Divider_Registers.html#ff07--tac-timer-control
        let pos: DWord = match self.clock_select() {
            // 4096 Hz
            0b00 => 9,
            0b01 => 3,
            0b10 => 5,
            0b11 => 7,
            _ => unreachable!(),
        };
        let update_tima = prev.test(pos) && !cur.test(pos);
        if !update_tima {
            TickResult::Ok
        } else if self.tima == Word::MAX {
            self.tima = self.tma;
            TickResult::IntReq
        } else {
            self.tima += 1;
            TickResult::Ok
        }
    }
}
