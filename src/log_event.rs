use std::collections::HashMap;

use rust_extensions::date_time::DateTimeAsMicroseconds;

#[derive(Debug)]
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
}

#[derive(Debug)]
pub struct MyLogEvent {
    pub dt: DateTimeAsMicroseconds,
    pub level: LogLevel,
    pub process: String,
    pub message: String,
    pub context: Option<HashMap<String, String>>,
}
