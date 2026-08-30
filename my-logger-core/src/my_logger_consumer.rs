use std::sync::Arc;

use super::MyLogEvent;

pub trait MyLoggerConsumer {
    fn write_log(&self, log_event: Arc<MyLogEvent>);
}
