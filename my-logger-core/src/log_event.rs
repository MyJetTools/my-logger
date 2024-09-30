use std::collections::HashMap;

use rust_extensions::date_time::DateTimeAsMicroseconds;

#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    FatalError,
    Debug,
}

impl LogLevel {
    pub fn to_string(&self) -> &'static str {
        match self {
            LogLevel::Info => "Info",
            LogLevel::Warning => "Warning",
            LogLevel::Error => "Error",
            LogLevel::FatalError => "FatalError",
            LogLevel::Debug => "Debug",
        }
    }

    pub fn is_error_or_fatal_error(&self) -> bool {
        match self {
            LogLevel::Error => true,
            LogLevel::FatalError => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct MyLogEvent {
    pub dt: DateTimeAsMicroseconds,
    pub level: LogLevel,
    pub process: String,
    pub message: String,
    pub context: Option<HashMap<String, String>>,
}
