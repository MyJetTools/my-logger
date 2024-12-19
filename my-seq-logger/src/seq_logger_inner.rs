use std::sync::Arc;

use rust_extensions::events_loop::EventsLoopTick;

use crate::{FlUrlUploader, LogEventsQueue, SeqLoggerSettings, SeqSettings};

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

        let events = self.log_events.dequeue().await;

        if events.is_none() {
            return;
        }

        let events = events.unwrap();

        let populated_params = my_logger_core::LOGGER.get_populated_params().await;

        let fl_url_uploader = FlUrlUploader::new(settings.url, settings.api_key);

        crate::upload_logs_chunk::upload_log_events_chunk(
            &fl_url_uploader,
            populated_params,
            events,
        )
        .await;
    }
    async fn finished(&self) {}
}
