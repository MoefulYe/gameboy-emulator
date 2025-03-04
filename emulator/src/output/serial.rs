use crate::external::emulator_serial_callback;

pub trait SerialOutput {
    fn put_serial(&mut self, data: u8);
    fn flush(&mut self);
}

pub struct WebSerialOutput(Vec<u8>);

impl WebSerialOutput {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl SerialOutput for WebSerialOutput {
    fn put_serial(&mut self, data: u8) {
        self.0.push(data);
        if self.0.len() > 128 {
            emulator_serial_callback(&self.0);
            self.0.clear();
        }
    }

    fn flush(&mut self) {
        if self.0.len() > 0 {
            emulator_serial_callback(&self.0);
            self.0.clear();
        }
    }
}
