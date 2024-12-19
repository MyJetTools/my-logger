use rust_extensions::date_time::DateTimeAsMicroseconds;
use tokio::sync::Mutex;

use crate::{ConsoleFilter, LogLevel, LogReaders, LogsStatistics};

pub struct MyLoggerInner {
    pub console_printer: ConsoleFilter,
    pub statistics: LogsStatistics,
    pub log_readers: Mutex<LogReaders>,

    pub start_time: DateTimeAsMicroseconds,
}

impl MyLoggerInner {
    pub fn new(populated_params: Vec<(String, String)>) -> Self {
        Self {
            console_printer: ConsoleFilter::new(),
            statistics: LogsStatistics::new(),
            start_time: DateTimeAsMicroseconds::now(),
            log_readers: Mutex::new(LogReaders::new(populated_params)),
        }
    }

    pub fn update_statistics(&self, log_event: LogLevel) {
        match log_event {
            LogLevel::Info => {
                self.statistics
                    .info
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
            LogLevel::Warning => {
                self.statistics
                    .warnings
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
            LogLevel::Error => {
                self.statistics
                    .errors
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
            LogLevel::FatalError => {
                self.statistics
                    .fatal_errors
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
            LogLevel::Debug => {
                self.statistics
                    .debugs
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
        }
    }
}
