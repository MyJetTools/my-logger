use super::MyLogEvent;

pub trait MyLoggerReader {
    fn write_log(&self, log_event: &MyLogEvent);
}
