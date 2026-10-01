use std::sync::Arc;

use crate::{SeqLoggerInner, SeqLoggerSettings, SeqSettings};
use my_logger_core::{LogEventCtx, MyLogEvent, MyLoggerConsumer};
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
            background_executor: BackgroundExecutor::new("SeqLogger", my_logger_core::LOGGER.clone()),
        };

        result.background_executor.register(result.inner.clone());

        // Spawns the one reader task onto the runtime we are being awaited on -
        // the host's. It lives as long as that runtime does, so the caller has to
        // keep it alive for as long as it wants logs shipped.
        //
        // Writing a log needs none of this: `trigger` only adds a permit to a
        // semaphore, which is safe from any thread, with or without a runtime
        // around it - a Tokio task, a plain `std::thread`, or an OS thread a C++
        // host owns and calls in through FFI.
        result.background_executor.start();

        println!("Seq Logs writer is started");

        let seq_logger = Arc::new(result);
        my_logger_core::LOGGER.plug_consumer(seq_logger.clone());
    }
}

impl MyLoggerConsumer for SeqLogger {
    fn write_log(&self, log_event: Arc<MyLogEvent>) {
        self.inner.log_events.enqueue(log_event);
        self.background_executor.trigger();
    }
}
