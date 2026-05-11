use std::sync::Arc;

use my_logger_core::MyLogEvent;
use parking_lot::Mutex;

const INITIAL_QUEUE_CAPACITY: usize = 64;

pub struct LogEventsQueue {
    queue: Mutex<Vec<Arc<MyLogEvent>>>,
    queue_size: Option<usize>,
}

impl LogEventsQueue {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(Vec::with_capacity(INITIAL_QUEUE_CAPACITY)),
            queue_size: None,
        }
    }

    pub fn configure_size(&mut self, queue_size: usize) {
        self.queue_size = Some(queue_size);
    }

    pub fn enqueue(&self, log_event: Arc<MyLogEvent>) {
        let mut queue = self.queue.lock();
        if let Some(limit) = self.queue_size {
            if queue.len() >= limit {
                return;
            }
        }
        queue.push(log_event);
    }

    pub fn dequeue(&self) -> Option<Vec<Arc<MyLogEvent>>> {
        let mut queue = self.queue.lock();
        if queue.is_empty() {
            return None;
        }
        let cap = queue.capacity();
        Some(std::mem::replace(&mut *queue, Vec::with_capacity(cap)))
    }
}
