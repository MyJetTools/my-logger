use std::{
    collections::HashMap,
    sync::{atomic::AtomicU64, Arc},
};

use rust_extensions::{date_time::DateTimeAsMicroseconds, Logger, StrOrString};
use tokio::sync::Mutex;

use crate::{ConsoleFilter, LogEventCtx, MyLogEvent, MyLoggerInner, MyLoggerReader};

use super::LogLevel;

pub struct MyLogger {
    inner: Arc<Mutex<MyLoggerInner>>,
    debugs: AtomicU64,
    fatal_errors: AtomicU64,
    errors: AtomicU64,
    warnings: AtomicU64,
    info: AtomicU64,
    pub to_console_filter: ConsoleFilter,
}

impl MyLogger {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(MyLoggerInner::new(HashMap::new()))),
            to_console_filter: ConsoleFilter::new(),
            debugs: AtomicU64::new(0),
            fatal_errors: AtomicU64::new(0),
            errors: AtomicU64::new(0),
            warnings: AtomicU64::new(0),
            info: AtomicU64::new(0),
        }
    }

    pub async fn populate_app_and_version(
        &self,
        app_name: impl Into<StrOrString<'static>>,
        app_version: impl Into<StrOrString<'static>>,
    ) {
        let mut write_access = self.inner.lock().await;
        write_access.populate_params("Application".to_string(), app_name.into().to_string());
        write_access.populate_params("Version".to_string(), app_version.into().to_string());
    }

    pub async fn plug_reader(&self, reader: Arc<dyn MyLoggerReader + Send + Sync + 'static>) {
        let mut write_access = self.inner.lock().await;
        write_access.register_reader(reader);
    }

    pub async fn populate_params(
        &self,
        key: impl Into<StrOrString<'static>>,
        value: impl Into<StrOrString<'static>>,
    ) {
        let key: StrOrString<'static> = key.into();
        let value: StrOrString<'static> = value.into();

        let mut write_access = self.inner.lock().await;
        write_access.populate_params(key.to_string(), value.to_string());
    }

    pub async fn get_populated_params(&self) -> Option<HashMap<String, String>> {
        let read_access = self.inner.lock().await;

        let populated_params = read_access.get_populated_params();

        if populated_params.is_empty() {
            None
        } else {
            Some(populated_params.clone())
        }
    }

    pub fn write_log(
        &self,
        level: LogLevel,
        process: String,
        message: String,
        context: Option<HashMap<String, String>>,
    ) {
        match level {
            LogLevel::Info => {
                self.info.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
            LogLevel::Warning => {
                self.warnings
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
            LogLevel::Error => {
                self.errors
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
            LogLevel::FatalError => {
                self.fatal_errors
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
            LogLevel::Debug => {
                self.debugs
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
        }

        let log_event = MyLogEvent {
            dt: DateTimeAsMicroseconds::now(),
            context,
            level,
            message,
            process,
        };

        let inner = self.inner.clone();

        let print_infos = self
            .to_console_filter
            .print_infos
            .load(std::sync::atomic::Ordering::Relaxed);

        let print_warnings = self
            .to_console_filter
            .print_warnings
            .load(std::sync::atomic::Ordering::Relaxed);

        let print_errors = self
            .to_console_filter
            .print_errors
            .load(std::sync::atomic::Ordering::Relaxed);

        let print_fatal_errors = self
            .to_console_filter
            .print_fatal_errors
            .load(std::sync::atomic::Ordering::Relaxed);

        let print_debug = self
            .to_console_filter
            .print_debug
            .load(std::sync::atomic::Ordering::Relaxed);

        tokio::spawn(async move {
            let inner_read_access = inner.lock().await;
            match &log_event.level {
                LogLevel::Info => {
                    if print_infos {
                        write_log(&log_event).await;
                    }
                }
                LogLevel::Warning => {
                    if print_warnings {
                        write_log(&log_event).await;
                    }
                }
                LogLevel::Error => {
                    if print_errors {
                        write_log(&log_event).await;
                    }
                }
                LogLevel::FatalError => {
                    if print_fatal_errors {
                        write_log(&log_event).await;
                    }
                }

                LogLevel::Debug => {
                    if print_debug {
                        write_log(&log_event).await;
                    }
                }
            }

            let log_event = Arc::new(log_event);

            for reader in inner_read_access.get_readers() {
                reader.write_log(log_event.clone()).await;
            }
        });
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

    pub fn get_errors_amount(&self) -> u64 {
        self.errors.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn get_warnings_amount(&self) -> u64 {
        self.warnings.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn get_fatal_errors_amount(&self) -> u64 {
        self.fatal_errors.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn get_debugs_amount(&self) -> u64 {
        self.debugs.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn get_infos_amount(&self) -> u64 {
        self.info.load(std::sync::atomic::Ordering::Relaxed)
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

async fn write_log(log_event: &MyLogEvent) {
    println!("{} {:?}", log_event.dt.to_rfc3339(), log_event.level);
    println!("Process: {}", log_event.process);
    println!("Message: {}", log_event.message);

    if let Some(ctx) = &log_event.context {
        println!("Context: {:?}", ctx);
    }

    println!("-------------------")
}
