use std::{collections::HashMap, sync::Arc};

use rust_extensions::{date_time::DateTimeAsMicroseconds, Logger, StrOrString};

use crate::{LogEventCtx, MyLogEvent, MyLoggerInner, MyLoggerReader, PopulatedParams};

use super::LogLevel;

pub struct MyLogger {
    inner: Arc<MyLoggerInner>,
}

impl MyLogger {
    pub fn new() -> Self {
        let inner = Arc::new(MyLoggerInner::new(Vec::new()));
        Self { inner }
    }

    pub async fn populate_app_and_version(
        &self,
        app_name: &'static str,
        app_version: &'static str,
    ) {
        let inner = {
            let write_access = self.inner.log_readers.load();

            if let Ok(env_info) = std::env::var("ENV_INFO") {
                let params: [(&'static str, StrOrString<'static>); 3] = [
                    ("Application", app_name.into()),
                    ("Version", app_version.into()),
                    ("EnvInfo", env_info.into()),
                ];
                write_access.populate_params(params.into_iter())
            } else {
                let params = [
                    ("Application", app_name.into()),
                    ("Version", app_version.into()),
                ];
                write_access.populate_params(params.into_iter())
            }
        };

        self.inner.log_readers.store(inner.into());
    }

    pub fn plug_reader(&self, reader: Arc<dyn MyLoggerReader + Send + Sync + 'static>) {
        let inner = {
            let write_access = self.inner.log_readers.load();
            write_access.register_reader(reader)
        };

        self.inner.log_readers.store(inner.into());
    }

    pub fn populate_params(&self, key: &'static str, value: impl Into<StrOrString<'static>>) {
        let value: StrOrString<'static> = value.into();

        let inner = {
            let write_access = self.inner.log_readers.load();

            let items = [(key, value)];
            write_access.populate_params(items.into_iter())
        };

        self.inner.log_readers.store(inner.into());
    }

    pub fn get_populated_params(&self) -> PopulatedParams {
        let read_access = self.inner.log_readers.load();
        read_access.get_populated_params().clone()
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

        self.inner.update_statistics(log_event.level);
        self.inner.console_printer.print_to_console(&log_event);

        let readers = self.inner.log_readers.load();

        let log_event = Arc::new(log_event);
        for reader in readers.get_readers() {
            reader.write_log(log_event.clone());
        }
    }

    #[deprecated(note = "Use write_log instead")]
    pub async fn write_log_async(&self, log_event: Arc<MyLogEvent>) {
        let inner = self.inner.clone();
        self.inner.update_statistics(log_event.level);
        let inner_read_access = inner.log_readers.load();
        inner.console_printer.print_to_console(&log_event);

        for reader in inner_read_access.get_readers() {
            reader.write_log(log_event.clone());
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
