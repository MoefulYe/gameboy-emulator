use crate::{external::emulator_log_callback, types::Addr};
use log::{error, LevelFilter, Log};

pub struct EmulatorLogger;

impl EmulatorLogger {
    const fn new() -> Self {
        Self
    }
}

impl Log for EmulatorLogger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let level = record.level() as u8;
        let msg = record.args().to_string();
        emulator_log_callback(level, &msg)
    }

    fn flush(&self) {}
}

pub fn init_logger() {
    static LOGGER: EmulatorLogger = EmulatorLogger::new();
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(LevelFilter::Trace)
}
