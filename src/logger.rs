use std::sync::Arc;

use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{MyLogEvent, MyLoggerReaderToConcole};

use super::{LogLevel, MyLoggerReader};

pub struct MyLogger {
    logger_reader: Arc<dyn MyLoggerReader>,
}

impl MyLogger {
    pub fn new(logger_reader: Arc<dyn MyLoggerReader>) -> Self {
        Self { logger_reader }
    }

    pub fn to_concole() -> Self {
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
