use std::sync::Arc;

use rust_extensions::{date_time::DateTimeAsMicroseconds, Logger};

use crate::{MyLogEvent, MyLoggerReaderToConcole};

use super::{LogLevel, MyLoggerReader};

pub struct MyLogger {
    logger_reader: Arc<dyn MyLoggerReader + Sync + Send + 'static>,
}

impl MyLogger {
    pub fn new(logger_reader: Arc<dyn MyLoggerReader + Sync + Send + 'static>) -> Self {
        Self { logger_reader }
    }

    pub fn to_console() -> Self {
        Self {
            logger_reader: Arc::new(MyLoggerReaderToConcole::new()),
        }
    }

    pub fn write_log(
        &self,
        level: LogLevel,
        process: String,
        message: String,
        context: Option<String>,
    ) {
        let log_event = MyLogEvent {
            dt: DateTimeAsMicroseconds::now(),
            context,
            level,
            message,
            process,
        };

        self.logger_reader.write_log(log_event);
    }
}

impl Logger for MyLogger {
    fn write_info(&self, process: String, message: String, ctx: Option<String>) {
        self.write_log(LogLevel::Info, process, message, ctx);
    }

    fn write_warning(&self, process: String, message: String, ctx: Option<String>) {
        self.write_log(LogLevel::Warning, process, message, ctx);
    }

    fn write_error(&self, process: String, message: String, ctx: Option<String>) {
        self.write_log(LogLevel::Error, process, message, ctx);
    }

    fn write_fatal_error(&self, process: String, message: String, ctx: Option<String>) {
        self.write_log(LogLevel::FatalError, process, message, ctx);
    }
}
