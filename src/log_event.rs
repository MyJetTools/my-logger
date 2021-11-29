#[derive(Debug)]
pub enum LogType {
    Info,
    Error,
    FatalError,
}

impl LogType {
    pub fn to_string(&self) -> &'static str {
        match self {
            LogType::Info => "Info",
            LogType::Error => "Error",
            LogType::FatalError => "FatalError",
        }
    }
}

#[derive(Debug)]
pub struct MyLogEvent {
    pub log_type: LogType,
    pub process: String,
    pub message: String,
    pub context: Option<String>,
}
