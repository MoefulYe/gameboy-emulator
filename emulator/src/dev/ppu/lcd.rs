use serde::{Deserialize, Serialize};

use crate::types::Word;

use super::{graphic::PPU_XRES, PPU};

#[derive(Serialize, Deserialize)]
pub(super) struct LCDDriver {
    pub draw_x: Word,
}

impl Default for LCDDriver {
    fn default() -> Self {
        Self { draw_x: 0 }
    }
}

impl LCDDriver {
    pub(super) fn new() -> Self {
        Default::default()
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
        let bgw_pixel = unsafe { self.bgw_queue.pop_front().unwrap_unchecked() };
        let obj_pixel = unsafe { self.obj_queue.pop_front().unwrap_unchecked() };
        let bgw_color = bgw_pixel.final_color();
        let obj_color = obj_pixel.final_color();
        let draw_obj = obj_pixel.color != 0 && (!obj_pixel.bg_priority || bgw_color == 0);
        let final_color = if draw_obj { obj_color } else { bgw_color };
        let &rgba = unsafe { self.palette.get_unchecked(final_color as usize) };
        let x = self.lcd_driver.draw_x;
        let y = self.ly;
        unsafe {
            *self
                .current_buffer_mut()
                .get_unchecked_mut(y as usize)
                .get_unchecked_mut(x as usize) = rgba
        };
        self.lcd_driver.draw_x += 1;
    }
}
