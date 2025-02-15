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
    pub fn to_u8(&self) -> u8 {
        match self {
            LogLevel::Info => 0,
            LogLevel::Warning => 1,
            LogLevel::Error => 2,
            LogLevel::FatalError => 3,
            LogLevel::Debug => 4,
        }
    }

    pub fn from_u8(value: u8) -> Self {
        match value {
            1 => Self::Warning,
            2 => Self::Error,
            3 => Self::FatalError,
            4 => Self::Debug,
            _ => Self::Info,
        }
    }

    pub fn from_str(src: &str) -> Self {
        match src {
            "Warning" => Self::Warning,
            "Error" => Self::Error,
            "FatalError" => Self::FatalError,
            "Debug" => Self::Debug,
            _ => Self::Info,
        }
    }
    pub fn as_str(&self) -> &'static str {
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
