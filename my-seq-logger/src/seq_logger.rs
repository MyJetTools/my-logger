use std::{collections::HashMap, sync::Arc};

use my_logger_core::{LogEventCtx, MyLogEvent, MyLoggerReader};
use tokio::sync::Mutex;

use crate::{SeqLoggerSettings, SeqSettings};

pub struct SeqLogger {
    log_events: Arc<Mutex<Vec<Arc<MyLogEvent>>>>,
    settings: Arc<dyn SeqSettings + Send + Sync + 'static>,
}

impl SeqLogger {
    pub fn enable_from_connection_string(settings: Arc<dyn SeqSettings + Send + Sync + 'static>) {
        std::panic::set_hook(Box::new(|itm| {
            let ctx = if let Some(location) = itm.location() {
                LogEventCtx::new().add("Location", format!("{}", location))
            } else {
                LogEventCtx::new()
            };

            my_logger_core::LOGGER.write_fatal_error(
                "Panic Handler",
                format!("Panic info: {:?}", itm),
                ctx,
            );
        }));

        let seq_logger = Self {
            log_events: Arc::new(Mutex::new(Vec::new())),
            settings,
        };

        tokio::spawn(seq_logger.start());
    }

    async fn start(self) {
        let seq_logger = Arc::new(self);
        my_logger_core::LOGGER.plug_reader(seq_logger.clone()).await;

        let params = my_logger_core::LOGGER.get_populated_params().await;
        println!("Seq writer is started");
        read_log(seq_logger, params).await;
    }
}

#[async_trait::async_trait]
impl MyLoggerReader for SeqLogger {
    async fn write_log(&self, log_event: Arc<MyLogEvent>) {
        let mut write_access = self.log_events.lock().await;
        write_access.push(log_event);
    }
}

async fn read_log(logger: Arc<SeqLogger>, populated_params: Option<HashMap<String, String>>) {
    let conn_string = logger.settings.get_conn_string().await;
    let mut settings = SeqLoggerSettings::parse(conn_string.as_str()).await;

    let write_debug = std::env::var("DEBUG").is_ok();

    loop {
        let events = {
            let mut events = logger.log_events.lock().await;

            if events.len() == 0 {
                None
            } else {
                let mut result_events = Vec::new();

                while events.len() > 0 {
                    let event = events.remove(0);

                    if let my_logger_core::LogLevel::Debug = event.level {
                        if !write_debug {
                            continue;
                        }
                    }

                    result_events.push(event);

                    if result_events.len() >= settings.max_logs_flush_chunk {
                        break;
                    }
                }

                Some(result_events)
            }
        };

        match events {
            Some(events) => {
                let upload_result = super::sdk::push_logs_data(
                    settings.url.to_string(),
                    settings.api_key.as_ref(),
                    populated_params.as_ref(),
                    events,
                )
                .await;

                match upload_result {
                    Ok(_) => {}
                    Err(err) => {
                        println!("Error while uploading logs to seq. Err: {:?}", err);
                        let conn_string = logger.settings.get_conn_string().await;
                        settings = SeqLoggerSettings::parse(conn_string.as_str()).await;
                    }
                }
            }
            None => {
                tokio::time::sleep(settings.flush_delay).await;
            }
        }
    }
}
