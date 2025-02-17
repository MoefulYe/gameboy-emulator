use crate::types::Word;

use super::{graphic::PPU_XRES, PPU};

pub(super) struct LCDDriver {
    pub draw_x: Word,
}

impl LCDDriver {
    pub(super) fn new() -> Self {
        Self { draw_x: 0 }
    }
}

impl PPU {
    pub(super) fn lcd_draw_pixel(&mut self) {
        if self.bgw_queue.len() < 8 {
            return;
        }
        if self.lcd_driver.draw_x >= PPU_XRES {
            return;
        }
        // queue len >= 8 > 0
        let color = unsafe { self.bgw_queue.pop_front().unwrap_unchecked() };
        let &rgba = unsafe { self.palette.get_unchecked(color as usize) };
        let x = self.lcd_driver.draw_x;
        let y = self.ly;
        self.screen_buffer[y as usize][x as usize] = rgba;
        self.lcd_driver.draw_x = x + 1;
    }
}
