mod log_event;
mod logger;
mod logger_reader;

pub use log_event::{LogData, LogLevel, MyLogEvent};
pub use logger::MyLogger;
pub use logger_reader::{GetMyLoggerReader, MyLoggerReader};
