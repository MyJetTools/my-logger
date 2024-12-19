mod console_printer;
mod log_event;
mod log_event_ctx;
mod log_readers;
mod log_statistics;
mod logger;
mod logger_reader;
mod my_logger_inner;
pub use console_printer::*;
pub use log_event::{LogLevel, MyLogEvent};
pub use log_event_ctx::*;
pub use log_readers::*;
pub use log_statistics::*;
pub use logger::MyLogger;
pub use logger_reader::MyLoggerReader;
pub use my_logger_inner::*;
mod populated_params;
pub use populated_params::*;
use std::sync::Arc;

lazy_static::lazy_static! {
    pub static ref LOGGER: Arc<MyLogger> = {
        Arc::new(MyLogger::new())
    };
}
