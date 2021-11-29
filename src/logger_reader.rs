use std::sync::Arc;

use tokio::sync::Mutex;

use crate::logger::LoggerQueue;

use super::MyLogEvent;

pub struct MyLoggerReader {
    queue: Arc<Mutex<LoggerQueue>>,
}

impl MyLoggerReader {
    pub fn new(queue: Arc<Mutex<LoggerQueue>>) -> Self {
        Self { queue }
    }
    pub async fn get_next_line(&self, max_amount: usize) -> Option<Vec<MyLogEvent>> {
        let mut queue = self.queue.lock().await;

        queue.has_reader = true;

        if queue.data.len() == 0 {
            return None;
        }

        let mut result = Vec::with_capacity(max_amount);

        for item in queue.data.drain(..) {
            result.push(item);

            if result.len() >= max_amount {
                break;
            }
        }

        Some(result)
    }
}
