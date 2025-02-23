use serde::{Deserialize, Serialize};

use crate::{
    types::{Addr, Word},
    utils::bits::{BitMap, BitProxy},
};

const SEC: i64 = 1000;
const MIN: i64 = SEC * 60;
const HOUR: i64 = MIN * 60;
const DAY: i64 = HOUR * 24;
const OVERFLOW: i64 = DAY * 512;

#[derive(Serialize, Deserialize)]
pub struct RTC {
    sec: Word,
    min: Word,
    hour: Word,
    dl: Word,
    dh: Word,
    epoch: i64,
    time: i64,
    latching: bool,
    latched: bool,
}

impl RTC {
    pub fn new(epoch: i64) -> Self {
        Self {
            sec: 0,
            min: 0,
            hour: 0,
            dl: 0,
            dh: 0,
            epoch,
            time: epoch,
            latching: false,
            latched: false,
        }
    }
    pub fn read(&self, addr: Addr) -> Word {
        match addr {
            0x08 => self.sec,
            0x09 => self.min,
            0x0A => self.hour,
            0x0B => self.dl,
            0x0C => self.dh,
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, addr: Addr, data: Word) {
        match addr {
            0x08 => self.sec = data,
            0x09 => self.min = data,
            0x0A => self.hour = data,
            0x0B => self.dl = data,
            0x0C => self.dh = data,
            _ => unreachable!(),
        }
        self.update_epoch()
    }

    pub fn set_latch(&mut self, data: Word) {
        match data {
            0x00 => self.latching = true,
            0x01 => {
                if self.latching {
                    if !self.latched {
                        self.latched = true;
                    } else {
                        self.latched = false;
                        self.update_time_regs();
                    }
                }
            }
            _ => self.latching = false,
        }
    }

    pub fn update(&mut self, timestamp_ms: i64) {
        if !self.halt() {
            self.time = timestamp_ms;
            if !self.latched {
                self.update_time_regs();
            }
        }
    }

    fn update_time_regs(&mut self) {
        let ms = self.time - self.epoch;
        let sec = ms / 1000;
        self.sec = (sec % 60) as Word;
        let min = sec / 60;
        self.min = (min % 60) as Word;
        let hour = min / 60;
        self.hour = (hour % 24) as Word;
        let day = hour / 24;
        self.dl = (day & 0xFF) as Word;
        self.day_master_bit_mut().setval(day & 0x100 != 0);
        self.day_overflow_mut().setval(day >= 512);
    }

    fn days(&self) -> u16 {
        (self.dl as u16) | (self.dh.at(0) as u16) << 8
    }

    fn day_overflow_mut(&mut self) -> BitProxy {
        BitProxy::new(&mut self.dh, 7)
    }

    fn day_overflow(&self) -> bool {
        self.dh.test(7)
    }

    fn halt_mut(&mut self) -> BitProxy {
        BitProxy::new(&mut self.dh, 6)
    }

    fn halt(&self) -> bool {
        self.dh.test(6)
    }

    fn day_master_bit_mut(&mut self) -> BitProxy {
        BitProxy::new(&mut self.dh, 0)
    }

    fn day_master_bit(&self) -> bool {
        self.dh.test(0)
    }

    fn update_epoch(&mut self) {
        let duration = self.sec as i64 * SEC
            + self.min as i64 * MIN
            + self.hour as i64 * HOUR
            + self.days() as i64 * HOUR
            + if self.day_overflow() { 1 } else { 0 } * OVERFLOW;
        self.epoch = self.time - duration;
    }
}
