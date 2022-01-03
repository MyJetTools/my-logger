use std::{collections::VecDeque, sync::Arc};

use rust_extensions::date_time::DateTimeAsMicroseconds;
use tokio::sync::Mutex;

use crate::{LogData, MyLogEvent};

use super::{LogLevel, MyLoggerReader};

pub enum MyLogger {
    ToConcole,
    ToReader(Arc<Mutex<VecDeque<MyLogEvent>>>),
}

impl MyLogger {
    pub fn new(logger_reader: Option<&MyLoggerReader>) -> Self {
        if let Some(logger_reader) = logger_reader {
            Self::ToReader(logger_reader.get_queue_for_writer())
        } else {
            Self::ToConcole
        }
    }

    pub fn shutdown(&self) {
        match self {
            Self::ToConcole => {}
            Self::ToReader(queue) => {
                let queue = queue.clone();
                tokio::spawn(async move {
                    let mut write_access = queue.lock().await;
                    write_access.push_back(MyLogEvent::TheEnd);
                });
            }
        }
    }

    pub fn write_log(
        &self,
        level: LogLevel,
        process: String,
        message: String,
        context: Option<String>,
    ) {
        let dt = DateTimeAsMicroseconds::now();
        match self {
            Self::ToConcole => {
                println!("{} {:?}", dt.to_rfc3339(), level);
                println!("Process: {}", process);
                println!("Message: {}", message);

                if let Some(ctx) = context {
                    println!("Context: {}", ctx);
                }
            }
            Self::ToReader(queue) => {
                let queue = queue.clone();
                tokio::spawn(async move {
                    let mut write_access = queue.lock().await;
                    write_access.push_back(MyLogEvent::NewEvent(LogData {
                        dt,
                        context,
                        level,
                        message,
                        process,
                    }));
                });
            }
        }
    }
}
