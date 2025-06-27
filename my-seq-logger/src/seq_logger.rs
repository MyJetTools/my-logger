use std::sync::Arc;

use crate::{SeqLoggerInner, SeqSettings};
use my_logger_core::{LogEventCtx, MyLogEvent, MyLoggerReader};
use rust_extensions::{events_loop::EventsLoop, AppStates};

pub struct SeqLogger {
    inner: Arc<SeqLoggerInner>,
    events_loop: EventsLoop<()>,
    app_states: Arc<AppStates>,
}

impl SeqLogger {
    pub async fn enable_from_connection_string(
        settings: Arc<dyn SeqSettings + Send + Sync + 'static>,
    ) {
        std::panic::set_hook(Box::new(|itm| {
            let ctx = if let Some(location) = itm.location() {
                LogEventCtx::new().add("Location", format!("{}", location))
            } else {
                LogEventCtx::new()
            };

            let payload = itm.payload();

            if let Some(s) = payload.downcast_ref::<&str>() {
                println!("panic occurred: {s:?}");
            } else if let Some(s) = payload.downcast_ref::<String>() {
                println!("panic occurred: {s:?}");
            } else {
                println!("panic occurred: {:?}", payload);
            }

            my_logger_core::LOGGER.write_fatal_error(
                "Panic Handler",
                format!("Panic info: {:?}", itm),
                ctx,
            );
        }));

        let mut result = Self {
            app_states: AppStates::create_initialized().into(),
            inner: Arc::new(SeqLoggerInner::new(settings)),
            events_loop: EventsLoop::new("SeqLogger".to_string(), my_logger_core::LOGGER.clone()),
        };

        result.events_loop.register_event_loop(result.inner.clone());

        result.events_loop.start(result.app_states.clone());

        let seq_logger = Arc::new(result);
        my_logger_core::LOGGER.plug_reader(seq_logger.clone()).await;
    }
}

#[async_trait::async_trait]
impl MyLoggerReader for SeqLogger {
    async fn write_log(&self, log_event: Arc<MyLogEvent>) {
        self.inner.log_events.enqueue(log_event).await;
        self.events_loop.send(());
    }
}
