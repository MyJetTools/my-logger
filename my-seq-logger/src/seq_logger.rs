use std::{collections::HashMap, sync::Arc, time::Duration};

use my_logger_core::{LogEventCtx, MyLogEvent, MyLoggerReader};
use tokio::sync::Mutex;

use crate::{SeqLoggerSettings, SeqSettings};

pub struct SeqLogger {
    log_events: Arc<Mutex<Option<Vec<Arc<MyLogEvent>>>>>,
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
            log_events: Arc::new(Mutex::new(None)),
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
        if write_access.is_none() {
            let mut new_vec = Vec::new();
            new_vec.reserve_exact(16);
            *write_access = Some(new_vec);
        }
        write_access.as_mut().unwrap().push(log_event);
    }
}

async fn read_log(logger: Arc<SeqLogger>, populated_params: Option<HashMap<String, String>>) {
    let mut conn_string = logger.settings.get_conn_string().await;
    let mut settings = SeqLoggerSettings::parse(conn_string.as_str()).await;
    loop {
        let events = {
            let mut events = logger.log_events.lock().await;

            if events.is_none() {
                None
            } else {
                events.take()
            }
        };

        if events.is_none() {
            tokio::time::sleep(settings.flush_delay).await;
            continue;
        }

        let seq_debug = std::env::var("SEQ_DEBUG").is_ok();

        let events = events.unwrap();

        push_it_out(events, &mut settings, seq_debug, populated_params.as_ref()).await;

        conn_string = logger.settings.get_conn_string().await;
        settings = SeqLoggerSettings::parse(conn_string.as_str()).await;
    }
}

async fn push_it_out(
    events: Vec<Arc<MyLogEvent>>,
    settings: &mut SeqLoggerSettings,
    seq_debug: bool,
    populated_params: Option<&HashMap<String, String>>,
) {
    let mut amount = 0;
    loop {
        amount += 1;
        if events.len() <= settings.max_logs_flush_chunk {
            let upload_result = super::sdk::push_logs_data(
                &settings.url,
                settings.api_key.as_ref(),
                populated_params,
                &events,
                seq_debug,
            )
            .await;

            if upload_result.is_ok() {
                break;
            }

            if amount > 3 {
                if let Err(err) = upload_result {
                    println!("Error while uploading logs to seq. Err: {:?}", err);
                }
                break;
            }
        } else {
            for chunk in events.chunks(settings.max_logs_flush_chunk) {
                let upload_result = super::sdk::push_logs_data(
                    &settings.url,
                    settings.api_key.as_ref(),
                    populated_params,
                    chunk,
                    seq_debug,
                )
                .await;

                if upload_result.is_ok() {
                    return;
                }

                if amount > 3 {
                    if let Err(err) = upload_result {
                        println!("Error while uploading logs to seq. Err: {:?}", err);
                    }
                    return;
                }
            }
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
