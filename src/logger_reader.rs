use super::MyLogEvent;

pub trait MyLoggerReader {
    fn write_log(&self, log_event: MyLogEvent);
}

pub struct MyLoggerReaderToConcole {}

impl MyLoggerReaderToConcole {
    pub fn new() -> Self {
        Self {}
    }
}

impl MyLoggerReader for MyLoggerReaderToConcole {
    fn write_log(&self, log_event: MyLogEvent) {
        println!("{} {:?}", log_event.dt.to_rfc3339(), log_event.level);
        println!("Process: {}", log_event.process);
        println!("Message: {}", log_event.message);

        if let Some(ctx) = log_event.context {
            println!("Context: {:?}", ctx);
        }

        println!("-------------------")
    }
}
