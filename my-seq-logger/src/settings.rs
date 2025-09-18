use std::{sync::Arc, time::Duration};

#[async_trait::async_trait]
pub trait SeqSettings {
    async fn get_conn_string(&self) -> String;
}

const DEFAULT_FLUSH_SLEEP: u64 = 1;
const DEFAULT_FLUSH_CHUNK: usize = 50;

pub struct SeqLoggerSettings {
    pub url: String,
    pub api_key: Option<String>,
    pub max_logs_flush_chunk: usize,
    pub flush_delay: Duration,
    pub queue_size: Option<usize>,
}

impl SeqLoggerSettings {
    pub async fn read(settings: &Arc<dyn SeqSettings + Send + Sync + 'static>) -> Self {
        loop {
            let conn_string = settings.get_conn_string().await;
            let settings = SeqLoggerSettings::try_parse(conn_string.as_str());

            match settings {
                Ok(result) => return result,
                Err(err) => {
                    println!("Can not parse Logs settings. Err: {:?}", err);
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            };
        }
    }
    pub fn try_parse(conn_string: &str) -> Result<Self, String> {
        let mut url = None;
        let mut api_key = None;
        let mut max_logs_flush_chunk = DEFAULT_FLUSH_CHUNK;
        let mut flush_delay = DEFAULT_FLUSH_SLEEP;
        let mut queue_size = None;

        for item in conn_string.split(';') {
            let (key, value) = spit_key_value(item);

            match key {
                "url" => {
                    url = Some(value);
                }
                "apikey" => {
                    api_key = Some(value.to_string());
                }
                "flushlogschunk" => {
                    max_logs_flush_chunk = value
                        .parse::<usize>()
                        .expect("FlushLogsChunk must be a number");
                }
                "flushdelay" => {
                    flush_delay = value.parse::<u64>().expect("FlushDelay must be a number");
                }
                "queuesize" => {
                    queue_size = Some(value.parse::<usize>().expect("QueueSize must be a number"));
                }
                _ => {
                    panic!("Invalid key {} of seq connection string ", key);
                }
            }
        }

        if url.is_none() {
            return Err(format!(
                "There is no URL parameter in seq connection string",
            ));
        }

        let result = Self {
            url: url.unwrap().to_string(),
            api_key: api_key.to_owned(),
            max_logs_flush_chunk,
            flush_delay: Duration::from_secs(flush_delay),
            queue_size,
        };

        Ok(result)
    }
}

fn spit_key_value(str: &str) -> (&str, &str) {
    let index = str.find('=');

    if index.is_none() {
        panic!("Invalid {} key value of seq connection string", str);
    }

    let index = index.unwrap();

    return (&str[..index], &str[index + 1..]);
}

#[cfg(test)]
mod tests {
    use super::spit_key_value;
    use super::SeqLoggerSettings;
    use std::time::Duration;

    #[test]
    fn test_split_key_value() {
        let str = "A=B";

        let (key, value) = spit_key_value(str);

        assert_eq!("A", key);
        assert_eq!("B", value);
    }

    #[test]
    fn test_split_key_value_empty_value() {
        let str = "A=";

        let (key, value) = spit_key_value(str);

        assert_eq!("A", key);
        assert_eq!("", value);
    }

    #[test]
    fn test_try_parse_correct_correct_values() {
        let str = "url=seq.test.com;apikey=value;flushlogschunk=100;flushdelay=1;queuesize=10";

        let result = SeqLoggerSettings::try_parse(str);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!("seq.test.com", result.url);
        assert_eq!(Some("value".to_string()), result.api_key);
        assert_eq!(100, result.max_logs_flush_chunk);
        assert_eq!(Duration::from_secs(1u64), result.flush_delay);
        assert_eq!(Some(10), result.queue_size);
    }

    #[test]
    fn test_try_parse_default_values() {
        let str = "url=seq.test.com";

        let result = SeqLoggerSettings::try_parse(str);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!("seq.test.com", result.url);
        assert!(result.api_key.is_none());
        assert_eq!(super::DEFAULT_FLUSH_CHUNK, result.max_logs_flush_chunk);
        assert_eq!(super::DEFAULT_FLUSH_SLEEP, result.flush_delay.as_secs());
        assert!(result.queue_size.is_none())
    }
}
