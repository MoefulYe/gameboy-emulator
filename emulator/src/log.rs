use log::Log;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log(msg: &str);
    #[wasm_bindgen(js_namespace = console, js_name = warn)]
    fn warn(msg: &str);
    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn error(msg: &str);
    #[wasm_bindgen(js_namespace = console, js_name = info)]
    fn info(msg: &str);
    #[wasm_bindgen(js_namespace = console, js_name = debug)]
    fn debug(msg: &str);
}

struct WasmLogger;

impl Log for WasmLogger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        match record.metadata().level() {
            log::Level::Error => error(&record.args().to_string()),
            log::Level::Warn => warn(&record.args().to_string()),
            log::Level::Info => info(&record.args().to_string()),
            log::Level::Debug => debug(&record.args().to_string()),
            log::Level::Trace => debug(&record.args().to_string()),
        }
    }

    fn flush(&self) {}
}

struct NativeLogger;

impl Log for NativeLogger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        println!("[{}] {}", record.level(), record.args());
    }

    fn flush(&self) {}
}

pub fn init() -> Result<(), log::SetLoggerError> {
    if cfg!(target_family = "wasm32") {
        static LOGGER: WasmLogger = WasmLogger;
        log::set_logger(&LOGGER)
    } else {
        static LOGGER: NativeLogger = NativeLogger;
        log::set_logger(&LOGGER)
    }
}
