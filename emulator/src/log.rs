use crate::external::emulator_log_callback;
use log::{LevelFilter, Log};

pub struct EmulatorLogger;

impl EmulatorLogger {
    const fn new() -> Self {
        Self
    }

    const LOG_LEVELS: &'static [&'static str; 6] =
        &["off", "error", "warn", "info", "debug", "trace"];
}

impl Log for EmulatorLogger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let level = Self::LOG_LEVELS[record.level() as usize];
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
