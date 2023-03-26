mod log_event;
mod log_event_ctx;
mod logger;
mod logger_reader;
pub use log_event::{LogLevel, MyLogEvent};
pub use log_event_ctx::*;
pub use logger::MyLogger;
pub use logger_reader::MyLoggerReader;
use std::sync::Arc;

lazy_static::lazy_static! {
    pub static ref LOGGER: Arc<MyLogger> = {
        Arc::new(MyLogger::new())
    };

}
