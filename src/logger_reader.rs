use std::{collections::VecDeque, sync::Arc};

use tokio::sync::Mutex;

use super::MyLogEvent;

pub struct MyLoggerReader {
    queue: Arc<Mutex<VecDeque<MyLogEvent>>>,
}

impl MyLoggerReader {
    pub fn new(queue: Arc<Mutex<VecDeque<MyLogEvent>>>) -> Self {
        Self { queue }
    }
    pub async fn get_next_line(&self, max_amount: usize) -> Option<Vec<MyLogEvent>> {
        let mut queue = self.queue.lock().await;

        if queue.len() == 0 {
            return None;
        }

        let mut result = Vec::with_capacity(max_amount);

        for item in queue.drain(..) {
            result.push(item);

            if result.len() >= max_amount {
                break;
            }
        }

        Some(result)
    }
}
