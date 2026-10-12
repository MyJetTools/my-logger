use std::sync::Arc;

use parking_lot::Mutex;
use rust_extensions::background_executor::{BackgroundJob, RepeatIteration};

use crate::{FlUrlUploader, LogEventsQueue, SeqLoggerSettings, SeqSettings};

pub struct SeqLoggerInner {
    pub(crate) log_events: LogEventsQueue,
    settings: Arc<dyn SeqSettings + Send + Sync + 'static>,
    cached_uploader: Mutex<Option<Arc<FlUrlUploader>>>,
}

impl SeqLoggerInner {
    pub fn new(settings: Arc<dyn SeqSettings + Send + Sync + 'static>) -> Self {
        Self {
            log_events: LogEventsQueue::new(),
            settings,
            cached_uploader: Mutex::new(None),
        }
    }

    pub fn configure(&mut self, queue_size: usize) {
        self.log_events.configure_size(queue_size);
    }

    async fn get_uploader(&self) -> Arc<FlUrlUploader> {
        let settings = SeqLoggerSettings::read(&self.settings).await;

        let mut cached = self.cached_uploader.lock();
        if let Some(existing) = cached.as_ref() {
            if existing.matches(
                &settings.url,
                &settings.api_key,
                &settings.product,
                settings.timeout,
            ) {
                return existing.clone();
            }
        }

        let uploader = Arc::new(FlUrlUploader::new(
            settings.url,
            settings.api_key,
            settings.product,
            settings.timeout,
        ));
        *cached = Some(uploader.clone());
        uploader
    }
}

#[async_trait::async_trait]
impl BackgroundJob for SeqLoggerInner {
    async fn execute(&self) -> RepeatIteration {
        let events = match self.log_events.dequeue() {
            Some(events) => events,
            None => return RepeatIteration::No,
        };

        let uploader = self.get_uploader().await;
        let populated_params = my_logger_core::LOGGER.get_populated_params();

        crate::upload_logs_chunk::upload_log_events_chunk(
            uploader.as_ref(),
            populated_params,
            events,
        )
        .await;

        RepeatIteration::No
    }
}
