use std::{
    collections::HashMap,
    sync::{atomic::AtomicBool, Arc},
};

use rust_extensions::{date_time::DateTimeAsMicroseconds, Logger, StrOrString};
use tokio::sync::Mutex;

use crate::{LogEventCtx, MyLogEvent, MyLoggerReader};

use super::LogLevel;

pub struct ConsoleFilter {
    pub print_debug: AtomicBool,
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
            print_warnings: AtomicBool::new(true),
            print_infos: AtomicBool::new(true),
            print_debug: AtomicBool::new(true),
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

    pub fn set_print_debug(&self, value: bool) {
        self.print_debug
            .store(value, std::sync::atomic::Ordering::SeqCst);
    }
}

struct MyLoggerSingleThreaded {
    readers: Vec<Arc<dyn MyLoggerReader + Send + Sync + 'static>>,
}

impl MyLoggerSingleThreaded {
    pub fn new() -> Self {
        Self {
            readers: Vec::new(),
        }
    }
}

pub struct MyLogger {
    single_threaded: Arc<Mutex<MyLoggerSingleThreaded>>,
    pub to_console_filter: ConsoleFilter,
    reader_is_plugged: AtomicBool,
}

impl MyLogger {
    pub fn new() -> Self {
        Self {
            single_threaded: Arc::new(Mutex::new(MyLoggerSingleThreaded::new())),
            to_console_filter: ConsoleFilter::new(),
            reader_is_plugged: AtomicBool::new(false),
        }
    }

    pub async fn plug_reader(&self, reader: Arc<dyn MyLoggerReader + Send + Sync + 'static>) {
        let mut write_access = self.single_threaded.lock().await;
        write_access.readers.push(reader);

        self.reader_is_plugged
            .store(true, std::sync::atomic::Ordering::SeqCst);
    }

    fn write_log(
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

            LogLevel::Debug => {
                if self
                    .to_console_filter
                    .print_debug
                    .load(std::sync::atomic::Ordering::Relaxed)
                {
                    write_log(&log_event)
                }
            }
        }

        if self
            .reader_is_plugged
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            let single_threaded = self.single_threaded.clone();
            let log_event = Arc::new(log_event);

            tokio::spawn(async move {
                let read_access = single_threaded.lock().await;
                for reader in &read_access.readers {
                    reader.write_log(log_event.clone()).await;
                }
            });
        }
    }

    pub fn write_info<'s>(
        &self,
        process: impl Into<StrOrString<'static>>,
        message: impl Into<StrOrString<'s>>,
        ctx: LogEventCtx,
    ) {
        self.write_log(
            LogLevel::Info,
            process.into().to_string(),
            message.into().to_string(),
            ctx.get_result(),
        );
    }

    pub fn write_warning<'s>(
        &self,
        process: impl Into<StrOrString<'static>>,
        message: impl Into<StrOrString<'s>>,
        ctx: LogEventCtx,
    ) {
        self.write_log(
            LogLevel::Warning,
            process.into().to_string(),
            message.into().to_string(),
            ctx.get_result(),
        );
    }

    pub fn write_error<'s>(
        &self,
        process: impl Into<StrOrString<'static>>,
        message: impl Into<StrOrString<'s>>,
        ctx: LogEventCtx,
    ) {
        self.write_log(
            LogLevel::Error,
            process.into().to_string(),
            message.into().to_string(),
            ctx.get_result(),
        );
    }

    pub fn write_fatal_error<'s>(
        &self,
        process: impl Into<StrOrString<'static>>,
        message: impl Into<StrOrString<'s>>,
        ctx: LogEventCtx,
    ) {
        self.write_log(
            LogLevel::FatalError,
            process.into().to_string(),
            message.into().to_string(),
            ctx.get_result(),
        );
    }

    pub fn write_fatal_debug<'s>(
        &self,
        process: impl Into<StrOrString<'static>>,
        message: impl Into<StrOrString<'s>>,
        ctx: LogEventCtx,
    ) {
        self.write_log(
            LogLevel::Debug,
            process.into().to_string(),
            message.into().to_string(),
            ctx.get_result(),
        );
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

    fn write_debug_info(
        &self,
        process: String,
        message: String,
        ctx: Option<HashMap<String, String>>,
    ) {
        self.write_log(LogLevel::Debug, process, message, ctx);
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
