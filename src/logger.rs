use std::{collections::VecDeque, sync::Arc};

use rust_extensions::date_time::DateTimeAsMicroseconds;
use tokio::sync::Mutex;

use crate::LogData;

use super::{LogLevel, MyLogEvent, MyLoggerReader};

pub struct MyLogger {
    queue: Option<Arc<Mutex<VecDeque<MyLogEvent>>>>,
}

impl MyLogger {
    pub fn new() -> Self {
        Self { queue: None }
    }

    pub fn get_reader(&mut self) -> MyLoggerReader {
        let queue = Arc::new(Mutex::new(VecDeque::new()));

        self.queue = Some(queue.clone());

        MyLoggerReader::new(queue)
    }

    pub fn shutdown(&self) {
        if let Some(queue) = &self.queue {
            let queue = queue.clone();
            tokio::spawn(async move {
                let mut queue = queue.lock().await;
                queue.push_back(MyLogEvent::TheEnd);
            });
        }
    }

    pub fn write_log(
        &self,
        level: LogLevel,
        process: String,
        message: String,
        context: Option<String>,
    ) {
        if let Some(queue) = &self.queue {
            let queue = queue.clone();
            tokio::spawn(async move {
                let data = LogData {
                    level,
                    process,
                    message,
                    context,
                };

                let mut queue = queue.lock().await;
                queue.push_back(MyLogEvent::NewEvent(data));
            });
        } else {
            let now = DateTimeAsMicroseconds::now();
            println!("{} {:?}", now.to_rfc3339(), level);
            println!("Process: {}", process);
            println!("Message: {}", message);

            if let Some(ctx) = context {
                println!("Context: {}", ctx);
            }
        }
    }
}
