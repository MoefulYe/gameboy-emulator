use std::sync::Mutex;

use crate::external::emulator_log_callback;
use log::{LevelFilter, Log};
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

pub struct EmulatorLogger(Mutex<Vec<LogItem>>);

impl EmulatorLogger {
    const fn new() -> Self {
        Self(Mutex::new(Vec::new()))
    }
}

impl Log for EmulatorLogger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let mut logs = self.0.lock().unwrap();
        logs.push((record.level(), record.args().to_string()).into());
        if logs.len() > 64 {
            emulator_log_callback(JsValue::from_serde(logs.as_slice()).unwrap());
            logs.clear();
        }
    }

    fn flush(&self) {
        let mut logs = self.0.lock().unwrap();
        emulator_log_callback(JsValue::from_serde(logs.as_slice()).unwrap());
        logs.clear();
    }
}

pub fn init_logger() {
    static LOGGER: EmulatorLogger = EmulatorLogger::new();
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(LevelFilter::Debug)
}
