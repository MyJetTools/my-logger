use std::{collections::HashMap, sync::Arc};

use rust_extensions::{date_time::DateTimeAsMicroseconds, Logger, StrOrString};

use crate::{LogEventCtx, MyLogEvent, MyLoggerInner, MyLoggerReader};

use super::LogLevel;

pub struct MyLogger {
    inner: Arc<MyLoggerInner>,
}

impl MyLogger {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(MyLoggerInner::new(HashMap::new())),
        }
    }

    pub async fn populate_app_and_version(
        &self,
        app_name: impl Into<StrOrString<'static>>,
        app_version: impl Into<StrOrString<'static>>,
    ) {
        let mut write_access = self.inner.log_readers.lock().await;
        write_access.populate_params("Application".to_string(), app_name.into().to_string());
        write_access.populate_params("Version".to_string(), app_version.into().to_string());
    }

    pub async fn plug_reader(&self, reader: Arc<dyn MyLoggerReader + Send + Sync + 'static>) {
        let mut write_access = self.inner.log_readers.lock().await;
        write_access.register_reader(reader);
    }

    pub async fn populate_params(
        &self,
        key: impl Into<StrOrString<'static>>,
        value: impl Into<StrOrString<'static>>,
    ) {
        let key: StrOrString<'static> = key.into();
        let value: StrOrString<'static> = value.into();

        let mut write_access = self.inner.log_readers.lock().await;
        write_access.populate_params(key.to_string(), value.to_string());
    }

    pub async fn get_populated_params(&self) -> Option<HashMap<String, String>> {
        let read_access = self.inner.log_readers.lock().await;

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
        let log_event = MyLogEvent {
            dt: DateTimeAsMicroseconds::now(),
            context,
            level,
            message,
            process,
        };

        let inner = self.inner.clone();

        tokio::spawn(async move {
            inner.update_statistics(log_event.level);
            let inner_read_access = inner.log_readers.lock().await;
            inner.console_printer.print_to_console(&log_event);
            let log_event = Arc::new(log_event);

            for reader in inner_read_access.get_readers() {
                reader.write_log(log_event.clone()).await;
            }
        });
    }

    pub async fn write_log_async(&self, log_event: Arc<MyLogEvent>) {
        let inner = self.inner.clone();
        self.inner.update_statistics(log_event.level);
        let inner_read_access = inner.log_readers.lock().await;
        inner.console_printer.print_to_console(&log_event);

        for reader in inner_read_access.get_readers() {
            reader.write_log(log_event.clone()).await;
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

    #[deprecated(note = "Use write_debug instead")]
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

    pub fn write_debug<'s>(
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
        self.inner
            .statistics
            .errors
            .load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn get_warnings_amount(&self) -> u64 {
        self.inner
            .statistics
            .warnings
            .load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn get_fatal_errors_amount(&self) -> u64 {
        self.inner
            .statistics
            .fatal_errors
            .load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn get_debugs_amount(&self) -> u64 {
        self.inner
            .statistics
            .debugs
            .load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn get_infos_amount(&self) -> u64 {
        self.inner
            .statistics
            .info
            .load(std::sync::atomic::Ordering::Relaxed)
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
