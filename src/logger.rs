use std::{
    collections::HashMap,
    sync::{atomic::AtomicBool, Arc},
};

use rust_extensions::{date_time::DateTimeAsMicroseconds, Logger};
use tokio::sync::{
    mpsc::{UnboundedReceiver, UnboundedSender},
    Mutex,
};

use crate::{MyLogEvent, MyLoggerReader};

use super::LogLevel;

pub struct ConsoleFilter {
    pub print_fatal_errors: AtomicBool,
    pub print_errors: AtomicBool,
    pub print_warnings: AtomicBool,
    pub print_infos: AtomicBool,
}

impl ConsoleFilter {
    pub fn new() -> Self {
        Self {
            print_fatal_errors: AtomicBool::new(true),
            print_errors: AtomicBool::new(true),
            print_warnings: AtomicBool::new(false),
            print_infos: AtomicBool::new(false),
        }
    }

    pub fn set_print_fatal_errors(&self, value: bool) {
        self.print_fatal_errors
            .store(value, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn set_print_errors(&self, value: bool) {
        self.print_errors
            .store(value, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn set_print_warnings(&self, value: bool) {
        self.print_warnings
            .store(value, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn set_print_infos(&self, value: bool) {
        self.print_infos
            .store(value, std::sync::atomic::Ordering::SeqCst);
    }
}

struct MyLoggerSingleThreaded {
    readers: Vec<Arc<dyn MyLoggerReader + Send + Sync + 'static>>,
    receiver: Option<UnboundedReceiver<MyLogEvent>>,
}

impl MyLoggerSingleThreaded {
    pub fn new(receiver: UnboundedReceiver<MyLogEvent>) -> Self {
        Self {
            readers: Vec::new(),
            receiver: Some(receiver),
        }
    }
}

pub struct MyLogger {
    single_threaded: Arc<Mutex<MyLoggerSingleThreaded>>,
    pub to_console_filter: ConsoleFilter,
    sender: UnboundedSender<MyLogEvent>,
    receiver_is_plugged: AtomicBool,
}

impl MyLogger {
    pub fn new() -> Self {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
        Self {
            single_threaded: Arc::new(Mutex::new(MyLoggerSingleThreaded::new(receiver))),
            to_console_filter: ConsoleFilter::new(),
            sender,
            receiver_is_plugged: AtomicBool::new(false),
        }
    }

    pub async fn plug_reader(&self, reader: Arc<dyn MyLoggerReader + Send + Sync + 'static>) {
        let mut write_access = self.single_threaded.lock().await;
        write_access.readers.push(reader);

        if let Some(reader) = write_access.receiver.take() {
            self.receiver_is_plugged
                .store(true, std::sync::atomic::Ordering::SeqCst);

            tokio::spawn(write_logs(write_access.readers.clone(), reader));
        }
    }

    pub fn write_log(
        &self,
        level: LogLevel,
        process: String,
        message: String,
        context: Option<HashMap<String, String>>,
    ) {
        let log_event = MyLogEvent {
            dt: DateTimeAsMicroseconds::now(),
            context,
            level,
            message,
            process,
        };

        match &log_event.level {
            LogLevel::Info => {
                if self
                    .to_console_filter
                    .print_infos
                    .load(std::sync::atomic::Ordering::Relaxed)
                {
                    write_log(&log_event)
                }
            }
            LogLevel::Warning => {
                if self
                    .to_console_filter
                    .print_warnings
                    .load(std::sync::atomic::Ordering::Relaxed)
                {
                    write_log(&log_event)
                }
            }
            LogLevel::Error => {
                if self
                    .to_console_filter
                    .print_errors
                    .load(std::sync::atomic::Ordering::Relaxed)
                {
                    write_log(&log_event)
                }
            }
            LogLevel::FatalError => {
                if self
                    .to_console_filter
                    .print_fatal_errors
                    .load(std::sync::atomic::Ordering::Relaxed)
                {
                    write_log(&log_event)
                }
            }
        }

        if self
            .receiver_is_plugged
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            if self.sender.send(log_event).is_err() {
                println!("Can not send event to log sender");
            }
        }
    }
}

impl Logger for MyLogger {
    fn write_info(&self, process: String, message: String, ctx: Option<HashMap<String, String>>) {
        self.write_log(LogLevel::Info, process, message, ctx);
    }

    fn write_warning(
        &self,
        process: String,
        message: String,
        ctx: Option<HashMap<String, String>>,
    ) {
        self.write_log(LogLevel::Warning, process, message, ctx);
    }

    fn write_error(&self, process: String, message: String, ctx: Option<HashMap<String, String>>) {
        self.write_log(LogLevel::Error, process, message, ctx);
    }

    fn write_fatal_error(
        &self,
        process: String,
        message: String,
        ctx: Option<HashMap<String, String>>,
    ) {
        self.write_log(LogLevel::FatalError, process, message, ctx);
    }
}

fn write_log(log_event: &MyLogEvent) {
    println!("{} {:?}", log_event.dt.to_rfc3339(), log_event.level);
    println!("Process: {}", log_event.process);
    println!("Message: {}", log_event.message);

    if let Some(ctx) = &log_event.context {
        println!("Context: {:?}", ctx);
    }

    println!("-------------------")
}

async fn write_logs(
    readers: Vec<Arc<dyn MyLoggerReader + Send + Sync + 'static>>,
    mut receiver: UnboundedReceiver<MyLogEvent>,
) {
    loop {
        if let Some(message) = tokio::sync::mpsc::UnboundedReceiver::recv(&mut receiver).await {
            let message = Arc::new(message);
            for reader in readers.iter() {
                let reader = reader.clone();
                let message = message.clone();
                tokio::spawn(async move {
                    reader.write_log(&message).await;
                });
            }
        }
    }
}
