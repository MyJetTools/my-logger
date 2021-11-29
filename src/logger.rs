use rust_extensions::date_time::DateTimeAsMicroseconds;
use tokio::sync::mpsc::UnboundedSender;

use super::{LogType, MyLogEvent, MyLoggerReader};

pub struct MyLogger {
    tx: Option<UnboundedSender<MyLogEvent>>,
}

impl MyLogger {
    pub fn new() -> Self {
        Self { tx: None }
    }

    pub fn get_reader(&mut self) -> MyLoggerReader {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        self.tx = Some(tx);
        MyLoggerReader::new(rx)
    }

    pub fn write_log(
        &self,
        log_type: LogType,
        process: String,
        message: String,
        context: Option<String>,
    ) {
        if let Some(tx) = &self.tx {
            let result = tx.send(MyLogEvent {
                log_type,
                process,
                message,
                context,
            });

            if let Err(err) = result {
                println!("Somehow we could not send log event to sender. Err:{}", err);
            }
        } else {
            let now = DateTimeAsMicroseconds::now();
            println!("{} {:?}", now.to_rfc3339(), log_type);
            println!("Process: {}", process);
            println!("Message: {}", message);

            if let Some(ctx) = context {
                println!("Context: {}", ctx);
            }
        }
    }
}
