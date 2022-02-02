use rust_extensions::date_time::DateTimeAsMicroseconds;

#[derive(Debug)]
pub enum LogLevel {
    Info,
    Error,
    FatalError,
}

impl LogLevel {
    pub fn to_string(&self) -> &'static str {
        match self {
            LogLevel::Info => "Info",
            LogLevel::Error => "Error",
            LogLevel::FatalError => "FatalError",
        }
    }
}

#[derive(Debug)]
pub struct MyLogEvent {
    pub dt: DateTimeAsMicroseconds,
    pub level: LogLevel,
    pub process: String,
    pub message: String,
    pub context: Option<String>,
}
