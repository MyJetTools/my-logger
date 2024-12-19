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

        match &mut *write_access {
            Some(queue) => {
                queue.push(log_event);
            }
            None => {
                *write_access = Some(vec![log_event]);
            }
        }
    }

    pub async fn dequeue(&self, max_amount: usize) -> Option<Vec<Arc<MyLogEvent>>> {
        let mut write_access = self.queue.lock().await;

        {
            let write_access = write_access.as_mut()?;

            if write_access.len() <= max_amount {
                let result: Vec<_> = write_access.drain(..max_amount).collect();
                return Some(result);
            }
        }

        write_access.take()
    }
}
