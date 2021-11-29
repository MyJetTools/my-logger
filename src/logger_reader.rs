use std::time::Duration;

use tokio::sync::mpsc::UnboundedReceiver;

use super::MyLogEvent;

pub struct MyLoggerReader {
    rx: UnboundedReceiver<MyLogEvent>,
}

impl MyLoggerReader {
    pub fn new(rx: UnboundedReceiver<MyLogEvent>) -> Self {
        Self { rx }
    }
    pub async fn get_next_line(&mut self) -> MyLogEvent {
        loop {
            let line = self.rx.recv().await;

            if let Some(event) = line {
                return event;
            } else {
                println!("Some how we did not get the log line");
                tokio::time::sleep(Duration::from_secs(3)).await;
            }
        }
    }
}
