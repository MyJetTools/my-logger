use std::sync::Arc;

use super::MyLogEvent;

#[async_trait::async_trait]
pub trait MyLoggerReader {
    async fn write_log(&self, log_event: Arc<MyLogEvent>);

    async fn has_no_events(&self) -> bool;

    async fn wait_until_everything_sent(&self) {
        loop {
            if self.has_no_events().await {
                break;
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    }
}
