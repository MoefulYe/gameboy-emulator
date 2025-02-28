use crate::external::emulator_serial_callback;

pub trait SerialOutput {
    fn put_serial(&mut self, data: u8);
}

pub struct WebSerialOutput;

impl WebSerialOutput {
    pub fn new() -> Self {
        Self
    }
}

impl SerialOutput for WebSerialOutput {
    fn put_serial(&mut self, data: u8) {
        emulator_serial_callback(data);
    }
}
