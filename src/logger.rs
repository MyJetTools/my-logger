use std::{collections::VecDeque, sync::Arc};

use rust_extensions::date_time::DateTimeAsMicroseconds;
use tokio::sync::Mutex;

use crate::LogData;

use super::{LogLevel, MyLogEvent, MyLoggerReader};

pub struct LoggerQueue {
    pub has_reader: bool,
    pub data: VecDeque<MyLogEvent>,
}

pub struct MyLogger {
    queue: Arc<Mutex<LoggerQueue>>,
}

impl MyLogger {
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(LoggerQueue {
                has_reader: false,
                data: VecDeque::new(),
            })),
        }
    }

    pub fn get_reader(&self) -> MyLoggerReader {
        MyLoggerReader::new(self.queue.clone())
    }

    pub fn shutdown(&self) {
        let queue = self.queue.clone();
        tokio::spawn(async move {
            let mut queue = queue.lock().await;
            if queue.has_reader {
                queue.data.push_back(MyLogEvent::TheEnd);
            }
        });
    }

    pub fn write_log(
        &self,
        level: LogLevel,
        process: String,
        message: String,
        context: Option<String>,
    ) {
        let queue = self.queue.clone();

        tokio::spawn(async move {
            let mut queue = queue.lock().await;

            if queue.has_reader {
                let data = LogData {
                    level,
                    process,
                    message,
                    context,
                    dt: DateTimeAsMicroseconds::now(),
                };

                queue.data.push_back(MyLogEvent::NewEvent(data));
            } else {
                let now = DateTimeAsMicroseconds::now();
                println!("{} {:?}", now.to_rfc3339(), level);
                println!("Process: {}", process);
                println!("Message: {}", message);

                if let Some(ctx) = context {
                    println!("Context: {}", ctx);
                }
            }
        });
    }
}
