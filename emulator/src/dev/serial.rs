use crate::{types::Word, utils::bits::BitMap};

/// ref https://gbdev.io/pandocs/Serial_Data_Transfer_(Link_Cable).html#ff02--sc-serial-transfer-control
pub struct Serial {
    data: Word,
    control: Word,
    inprogress: bool,
    has_transfered: u8,
}

impl Serial {
    pub fn new() -> Self {
        Self {
            data: 0xFF,
            control: 0x7C,
            inprogress: false,
            has_transfered: 0,
        }
    }

    fn transfer_enable(&self) -> bool {
        self.control.at(7) != 0
    }

    fn master(&self) -> bool {
        self.control.at(0) != 0
    }

    fn slave(&self) -> bool {
        self.control.at(0) == 0
    }

    fn begin_transfer(&mut self) {
        self.inprogress = true;
        self.has_transfered = 0;
    }
}
