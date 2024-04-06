mod console_filter;
mod log_event;
mod log_event_ctx;
mod logger;
mod logger_reader;
mod my_logger_inner;
pub use console_filter::*;
pub use log_event::{LogLevel, MyLogEvent};
pub use log_event_ctx::*;
pub use logger::MyLogger;
pub use logger_reader::MyLoggerReader;
pub use my_logger_inner::*;
use std::sync::Arc;

lazy_static::lazy_static! {
    pub static ref LOGGER: Arc<MyLogger> = {
        Arc::new(MyLogger::new())
    };
}
