#![cfg(all(target_family="wasm", feature="browser"))]

use log::{Record, Level, Metadata};
use log::SetLoggerError;

use wasm_bindgen::JsValue;
use web_sys::console;

struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let console_log: fn(&JsValue) = match record.level() {
                Level::Trace => console::trace_1 as fn(&JsValue),
                Level::Debug => console::debug_1 as fn(&JsValue),
                Level::Info => console::info_1 as fn(&JsValue),
                Level::Warn => console::warn_1 as fn(&JsValue),
                Level::Error => console::error_1 as fn(&JsValue)
            };
    
            let message: String = format!("{}", record.args());
            let message_js: JsValue = message.into();
            console_log(&message_js);
        }
    }

    fn flush(&self) {}
}

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&ConsoleLogger)?;
    log::set_max_level(log::LevelFilter::Trace);

    Ok(())
}