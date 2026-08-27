use std::sync::Arc;

use crate::{SeqLoggerInner, SeqLoggerSettings, SeqSettings};
use my_logger_core::{LogEventCtx, MyLogEvent, MyLoggerReader};
use rust_extensions::background_executor::BackgroundExecutor;

pub struct SeqLogger {
    inner: Arc<SeqLoggerInner>,
    background_executor: BackgroundExecutor,
}

impl SeqLogger {
    pub async fn enable_from_connection_string(
        settings: Arc<dyn SeqSettings + Send + Sync + 'static>,
    ) {
        std::panic::set_hook(Box::new(|itm| {
            let mut ctx = if let Some(location) = itm.location() {
                LogEventCtx::new().add("Location", format!("{}", location))
            } else {
                LogEventCtx::new()
            };

            ctx = ctx.add("PanicInfo", format!("{}", itm));

            let payload = itm.payload();

            let panic_message = if let Some(s) = payload.downcast_ref::<&str>() {
                format!("{s:?}")
            } else if let Some(s) = payload.downcast_ref::<String>() {
                format!("{s:?}")
            } else {
                format!("{:?}", payload)
            };

            my_logger_core::LOGGER.write_fatal_error("Panic Handler", panic_message, ctx);
        }));

        let mut inner = SeqLoggerInner::new(settings.clone());
        let settings = SeqLoggerSettings::read(&settings).await;

        if let Some(queue_size) = settings.queue_size {
            inner.configure(queue_size)
        }

        let result = Self {
            inner: Arc::new(inner),
            background_executor: BackgroundExecutor::new("SeqLogger"),
        };

        result.background_executor.register(result.inner.clone());

        result
            .background_executor
            .start(my_logger_core::LOGGER.clone());

        println!("Seq Logs writer is started");

        let seq_logger = Arc::new(result);
        my_logger_core::LOGGER.plug_reader(seq_logger.clone());
    }
}

impl MyLoggerReader for SeqLogger {
    fn write_log(&self, log_event: Arc<MyLogEvent>) {
        self.inner.log_events.enqueue(log_event);
        self.background_executor.trigger();
    }
}
