use std::time::Duration;

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
}

impl SeqLoggerSettings {
    pub async fn parse(conn_string: &str) -> Self {
        loop {
            match Self::try_parse(conn_string) {
                Ok(result) => {
                    return result;
                }
                Err(err) => {
                    println!(
                        "Error while parsing connection string. Err: {}. ConnString: {}",
                        err, conn_string
                    );
                }
            }
        }
    }

    fn try_parse(conn_string: &str) -> Result<Self, String> {
        let mut url = None;
        let mut api_key = None;
        let mut max_logs_flush_chunk = DEFAULT_FLUSH_CHUNK;
        let mut flush_delay = DEFAULT_FLUSH_SLEEP;

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
}
