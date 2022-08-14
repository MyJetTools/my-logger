mod log_event;
mod logger;
mod logger_reader;

pub use log_event::{LogLevel, MyLogEvent};
pub use logger::MyLogger;
pub use logger_reader::MyLoggerReader;

lazy_static::lazy_static! {
    pub static ref LOGGER: MyLogger = {
        MyLogger::new()
    };
}
