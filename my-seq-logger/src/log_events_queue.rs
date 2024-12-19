use std::sync::Arc;

use my_logger_core::MyLogEvent;
use tokio::sync::Mutex;

pub struct LogEventsQueue {
    queue: Mutex<Option<Vec<Arc<MyLogEvent>>>>,
}

impl LogEventsQueue {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(None),
        }
    }

    pub async fn enqueue(&self, log_event: Arc<MyLogEvent>) {
        let mut write_access = self.queue.lock().await;

        if let Some(queue) = &mut *write_access {
            queue.push(log_event);
            return;
        }

        let mut new_queue = Vec::new();
        new_queue.push(log_event);
        *write_access = Some(new_queue);
    }

    pub async fn dequeue(&self) -> Option<Vec<Arc<MyLogEvent>>> {
        let mut write_access = self.queue.lock().await;
        write_access.take()
    }
}
