mod log_event;
mod logger;
mod logger_reader;

use std::sync::Arc;

pub use log_event::{LogLevel, MyLogEvent};
pub use logger::MyLogger;
pub use logger_reader::MyLoggerReader;

lazy_static::lazy_static! {
    pub static ref LOGGER: Arc<MyLogger> = {
        Arc::new(MyLogger::new())
    };
}
