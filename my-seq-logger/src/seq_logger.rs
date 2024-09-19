use std::{collections::HashMap, sync::Arc, time::Duration};

use my_logger_core::{MyLogEvent, MyLoggerReader};
use rust_extensions::{date_time::DateTimeAsMicroseconds, UnsafeValue};
use tokio::sync::Mutex;

use crate::{SeqLoggerSettings, SeqSettings};

#[derive(Default)]
pub struct SeqLoggerInner {
    pub events: Option<Vec<Arc<MyLogEvent>>>,
    pub wait_events: Vec<Arc<MyLogEvent>>,
}

impl SeqLoggerInner {
    pub fn add_event(&mut self, log_event: Arc<MyLogEvent>) {
        self.wait_events.push(log_event.clone());

        match self.events.as_mut() {
            Some(events) => {
                events.push(log_event);
            }
            None => {
                self.events = Some(vec![log_event]);
            }
        }
    }

    pub fn gc(&mut self) {
        self.wait_events.retain(|itm| !itm.sent.get_value())
    }
}

pub struct SeqLogger {
    log_events: Arc<Mutex<SeqLoggerInner>>,
    settings: Arc<dyn SeqSettings + Send + Sync + 'static>,
}

impl SeqLogger {
    pub fn enable_from_connection_string(settings: Arc<dyn SeqSettings + Send + Sync + 'static>) {
        let started = DateTimeAsMicroseconds::now();
        std::panic::set_hook(Box::new(move |itm| {
            let context = if let Some(location) = itm.location() {
                let mut ctx = HashMap::new();
                ctx.insert("Location".to_string(), format!("{}", location));
                Some(ctx)
            } else {
                None
            };

            let event: Arc<_> = MyLogEvent {
                level: my_logger_core::LogLevel::FatalError,
                process: "Panic Handler".to_string(),
                message: format!("Panic info: {:?}", itm),
                context,
                dt: DateTimeAsMicroseconds::now(),
                sent: UnsafeValue::new(false),
            }
            .into();

            let event_cloned = event.clone();
            tokio::spawn(async move { my_logger_core::LOGGER.write_log_async(event_cloned) });

            let now = DateTimeAsMicroseconds::now();

            if now.duration_since(started).as_positive_or_zero() < Duration::from_secs(15) {
                let mut no = 0;
                while no < 10 {
                    if event.sent.get_value() {
                        break;
                    }
                    std::thread::sleep(Duration::from_millis(100));
                    no += 1;
                }
            }
        }));

        let seq_logger = Self {
            log_events: Arc::new(Mutex::new(SeqLoggerInner::default())),
            settings,
        };

        tokio::spawn(seq_logger.start());
    }

    async fn start(self) {
        let seq_logger = Arc::new(self);
        my_logger_core::LOGGER.plug_reader(seq_logger.clone()).await;

        let params = my_logger_core::LOGGER.get_populated_params().await;
        println!("Seq writer is started");
        to_server_push_loop(seq_logger, params).await;
    }
}

#[async_trait::async_trait]
impl MyLoggerReader for SeqLogger {
    async fn write_log(&self, log_event: Arc<MyLogEvent>) {
        let mut write_access = self.log_events.lock().await;
        write_access.add_event(log_event);
    }

    async fn has_no_events(&self) -> bool {
        let read_access = self.log_events.lock().await;
        read_access.wait_events.is_empty()
    }
}

async fn to_server_push_loop(
    logger: Arc<SeqLogger>,
    populated_params: Option<HashMap<String, String>>,
) {
    let mut conn_string = logger.settings.get_conn_string().await;
    let mut settings = SeqLoggerSettings::parse(conn_string.as_str()).await;
    let seq_debug = std::env::var("SEQ_DEBUG").is_ok();
    loop {
        let events: Option<Vec<Arc<MyLogEvent>>> = {
            let mut inner = logger.log_events.lock().await;
            inner.gc();
            inner.events.take()
        };

        if events.is_none() {
            tokio::time::sleep(settings.flush_delay).await;
            continue;
        }

        let events = events.unwrap();

        push_it_out(
            events.as_slice(),
            &mut settings,
            seq_debug,
            populated_params.as_ref(),
        )
        .await;

        for event in events {
            event.sent.set_value(true);
        }

        conn_string = logger.settings.get_conn_string().await;
        settings = SeqLoggerSettings::parse(conn_string.as_str()).await;
    }
}

async fn push_it_out(
    events: &[Arc<MyLogEvent>],
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
