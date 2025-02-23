use super::{
    int_regs::{IRQ, IRQ_NONE, IRQ_SERIAL},
    BusDevice, Reset, Tick,
};
use crate::{
    external::emulator_serial_callback,
    types::{Addr, Word},
    utils::bits::BitMap,
};
use log::warn;
use serde::{Deserialize, Serialize};

const SERIAL_CONTROL_ENABLE: Word = 7;
#[allow(unused)]
const SERIAL_CONTROL_SPEED: Word = 1;
#[allow(unused)]
const SERIAL_CONTROL_SELECT: Word = 0;
pub const SERIAL_DATA_REG_ADDR: Addr = 0xFF01;
pub const SERIAL_CONTROL_REG_ADDR: Addr = 0xFF02;
pub const SERIAL_ADDR_LOW_BOUND: Addr = SERIAL_DATA_REG_ADDR;
pub const SERIAL_ADDR_HIGH_BOUND_INCLUDED: Addr = SERIAL_CONTROL_REG_ADDR;
/// ref https://gbdev.io/pandocs/Serial_Data_Transfer_(Link_Cable).html#ff02--sc-serial-transfer-control
#[derive(Serialize, Deserialize)]
pub struct Serial {
    sb: Word,
    out: Word,
    sc: Word,
    inprogress: bool,
    has_transfered: u8,
    ticks: u32,
}

impl Default for Serial {
    fn default() -> Self {
        Self {
            sb: 0xFF,
            out: 0x00,
            sc: 0x7C,
            inprogress: false,
            has_transfered: 0,
            ticks: 0,
        }
    }
}

impl BusDevice for Serial {
    fn read(&self, addr: Addr) -> Word {
        match addr {
            SERIAL_DATA_REG_ADDR => self.sb,
            SERIAL_CONTROL_REG_ADDR => self.sc,
            _ => {
                warn!("illegal read from serial at address: 0x{addr:04X}");
                0xFF
            }
        }
    }

    fn write(&mut self, addr: Addr, data: Word) {
        match addr {
            SERIAL_DATA_REG_ADDR => self.sb = data,
            SERIAL_CONTROL_REG_ADDR => self.sc = data,
            _ => warn!("illegal write to serial at address: 0x{addr:04X}"),
        }
    }
}

impl Serial {
    pub fn new() -> Self {
        Default::default()
    }

    fn transfer_enable(&self) -> bool {
        self.sc.test(SERIAL_CONTROL_ENABLE)
    }

    fn master(&self) -> bool {
        self.sc.test(SERIAL_CONTROL_SELECT)
    }

    fn begin_transfer(&mut self) {
        self.inprogress = true;
        self.has_transfered = 0;
        self.out = self.sb;
    }

    fn transfer(&mut self) -> IRQ {
        self.sb = (self.sb << 1) | 1;
        self.has_transfered += 1;
        if self.has_transfered >= 8 {
            self.end_transfer();
            IRQ_SERIAL
        } else {
            IRQ_NONE
        }
    }

    fn end_transfer(&mut self) {
        self.sc = self.sc.clear_at(SERIAL_CONTROL_ENABLE);
        self.inprogress = false;
        emulator_serial_callback(self.out);
    }
}

impl Tick for Serial {
    fn tick(&mut self) -> IRQ {
        self.ticks = self.ticks.wrapping_add(1);
        if self.ticks % 512 != 0 {
            // 当 ticks不是512的倍数时， 串口设备不进行工作,
            // 使得串口设备的工作频率为 8192Hz
            IRQ_NONE
        } else if !self.inprogress && self.transfer_enable() && self.master() {
            // 当串口设备处于空闲状态，且串口设备使能，且为主设备时，开始传输
            self.begin_transfer();
            IRQ_NONE
        } else if self.inprogress {
            // 当串口设备处于传输状态时，继续传输
            self.transfer()
        } else {
            // 其他情况，串口设备不进行工作
            IRQ_NONE
        }
    }
}
