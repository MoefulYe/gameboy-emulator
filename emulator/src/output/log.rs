use std::cell::UnsafeCell;

use crate::external::{console_log, emulator_log_callback};
use log::{Level, LevelFilter, Log};
use serde::Serialize;
use tsify::{JsValueSerdeExt, Tsify};
use tsify_derive::LogItem;
use wasm_bindgen::JsValue;

#[allow(non_snake_case)]
mod tsify_derive {
    use log::Level;

    use super::*;
    #[derive(Serialize, Tsify)]
    #[tsify(into_wasm_abi)]
    pub struct LogItem {
        pub level: u8,
        pub msg: String,
    }

    impl From<(Level, String)> for LogItem {
        fn from((level, msg): (Level, String)) -> Self {
            LogItem {
                level: level as u8,
                msg,
            }
        }
    }
}

pub struct EmulatorLogger(UnsafeCell<Vec<LogItem>>);

unsafe impl Sync for EmulatorLogger {}

impl EmulatorLogger {
    const fn new() -> Self {
        Self(UnsafeCell::new(Vec::new()))
    }
}

impl Log for EmulatorLogger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        unsafe {
            let logs = &mut *self.0.get();
            logs.push((record.level(), record.args().to_string()).into());
            if logs.len() > 1024 || record.level() == Level::Error || record.level() == Level::Debug
            {
                emulator_log_callback(JsValue::from_serde(logs.as_slice()).unwrap());
                logs.clear();
            }
        }
    }

    fn flush(&self) {
        unsafe {
            let logs = &mut *self.0.get();
            if logs.len() > 0 {
                emulator_log_callback(JsValue::from_serde(logs.as_slice()).unwrap());
                logs.clear();
            }
        }
    }
}

static LOGGER: EmulatorLogger = EmulatorLogger::new();
pub fn log_flush() {
    LOGGER.flush();
}

pub fn init_logger() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(LevelFilter::Debug)
}
