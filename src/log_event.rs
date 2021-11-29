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
pub enum MyLogEvent {
    NewEvent(LogData),
    TheEnd,
}

#[derive(Debug)]
pub struct LogData {
    pub level: LogLevel,
    pub process: String,
    pub message: String,
    pub context: Option<String>,
}
