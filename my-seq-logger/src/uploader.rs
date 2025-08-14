use std::time::Duration;

use flurl::{body::FlUrlBody, *};

const NULL_PARAM: Option<&str> = None;

#[async_trait::async_trait]
pub trait LogsChunkUploader {
    async fn upload_chunk(&self, chunk: &[u8]);
}

pub struct FlUrlUploader {
    pub url: String,
    pub api_key: Option<String>,
    pub seq_debug: bool,
    pub compress: bool,
}

impl FlUrlUploader {
    pub fn new(url: String, api_key: Option<String>) -> Self {
        Self {
            url,
            api_key,
            seq_debug: std::env::var("SEQ_DEBUG").is_ok(),
            compress: std::env::var("SEQ_COMPRESS").is_ok(),
        }
    }
}

#[async_trait::async_trait]
impl LogsChunkUploader for FlUrlUploader {
    async fn upload_chunk(&self, chunk_to_upload: &[u8]) {
        let mut attempt_no = 0;
        loop {
            attempt_no += 1;
            if self.seq_debug {
                if chunk_to_upload.len() > 256 {
                    println!("Sending log len={}", chunk_to_upload.len());
                } else {
                    println!("Sending log: [{:?}]", std::str::from_utf8(&chunk_to_upload));
                }
            }

            let mut fl_url = FlUrl::new(self.url.as_str())
                .set_timeout(Duration::from_secs(10))
                .append_path_segment("api")
                .append_path_segment("events")
                .append_path_segment("raw")
                .with_header("Accept", "*/*")
                .with_header("Content-Type", "application/vnd.serilog.clef")
                .with_retries(3);

            if self.compress && chunk_to_upload.len() > 1024 * 1024 {
                fl_url = fl_url.compress();
            }

            if let Some(api_key) = self.api_key.as_ref() {
                fl_url = fl_url.with_header("X-Seq-ApiKey", api_key);
            };

            let response = fl_url
                .append_query_param("clef", NULL_PARAM)
                .post(FlUrlBody::Raw {
                    data: chunk_to_upload.to_vec(),
                    content_type: None,
                })
                .await;

            match response {
                Ok(mut response) => {
                    if self.seq_debug {
                        print_fl_url_response(&mut response).await;
                    }

                    if is_status_code_ok(response.get_status_code()) {
                        return;
                    }
                }
                Err(err) => {
                    println!(
                        "Attempt: {} Error while uploading logs to seq. Err: {:?}",
                        attempt_no, err
                    );

                    if attempt_no > 3 {
                        return;
                    }

                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }
    }
}

fn is_status_code_ok(status_code: u16) -> bool {
    status_code >= 200 && status_code < 210
}

async fn print_fl_url_response(response: &mut FlUrlResponse) {
    println!("Result: {}", response.get_status_code());
    let body = response.get_body_as_slice().await;
    let body = match body {
        Ok(body) => body,

        Err(err) => {
            println!("Error while reading response body. Err: {:?}", err);
            return;
        }
    };

    match std::str::from_utf8(&body) {
        Ok(body_as_str) => {
            println!("Body: {}", body_as_str);
        }
        Err(_) => {
            println!("Body is not a valid utf8 string. Len: {}", body.len());
        }
    }
}
