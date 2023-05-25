use std::os::raw::c_char;
use log::{Record, Level, Metadata, SetLoggerError};

pub struct ConsoleLogger;

extern "C" {
    pub fn trace(message: *const c_char);
    pub fn debug(message: *const c_char);
    pub fn info(message: *const c_char);
    pub fn warn(message: *const c_char);
    pub fn error(message: *const c_char);
}

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let console_log: unsafe extern "C" fn(*const c_char) = match record.level() {
                Level::Trace => trace as unsafe extern "C" fn(*const c_char),
                Level::Debug => debug as unsafe extern "C" fn(*const c_char),
                Level::Info => info as unsafe extern "C" fn(*const c_char),
                Level::Warn => warn as unsafe extern "C" fn(*const c_char),
                Level::Error => error as unsafe extern "C" fn(*const c_char)
            };
    
            unsafe {
                console_log(format!("{}\0", record.args()).as_ptr() as *const c_char);
            }
        }
    }

    fn flush(&self) {}
}

#[allow(dead_code)]
pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&ConsoleLogger)?;
    log::set_max_level(log::LevelFilter::Trace);

    Ok(())
}