use super::MyLogEvent;

#[async_trait::async_trait]
pub trait MyLoggerReader {
    async fn write_log(&self, log_event: &MyLogEvent);
}
