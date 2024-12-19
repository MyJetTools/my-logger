use std::sync::Arc;

use rust_extensions::events_loop::EventsLoopTick;

use crate::{LogEventsQueue, SeqLoggerSettings, SeqSettings};

pub struct SeqLoggerInner {
    pub(crate) log_events: LogEventsQueue,
    settings: Arc<dyn SeqSettings + Send + Sync + 'static>,
}

impl SeqLoggerInner {
    pub fn new(settings: Arc<dyn SeqSettings + Send + Sync + 'static>) -> Self {
        Self {
            log_events: LogEventsQueue::new(),
            settings,
        }
    }
}

#[async_trait::async_trait]
impl EventsLoopTick<()> for SeqLoggerInner {
    async fn started(&self) {
        println!("Seq Logs writer is started");
    }
    async fn tick(&self, _: ()) {
        let settings = SeqLoggerSettings::read(&self.settings).await;

        let events = self.log_events.dequeue(settings.max_logs_flush_chunk).await;

        if events.is_none() {
            return;
        }

        let seq_debug = std::env::var("SEQ_DEBUG").is_ok();

        let events = events.unwrap();

        let populated_params = my_logger_core::LOGGER.get_populated_params().await;

        crate::upload_logs_chunk::upload_log_events_chunk(
            &settings.url,
            settings.api_key,
            populated_params,
            events,
            seq_debug,
        )
        .await;
    }
    async fn finished(&self) {}
}
