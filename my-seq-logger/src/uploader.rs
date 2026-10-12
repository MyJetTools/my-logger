use std::time::Duration;

use flurl::{body::HttpRequestBody, *};

const NULL_PARAM: Option<&str> = None;

// The header my-logger-server reads the product scope of the records from
const PRODUCT_SCOPE_HEADER: &str = "scope";

#[async_trait::async_trait]
pub trait LogsChunkUploader {
    async fn upload_chunk(&self, chunk_to_upload: Vec<u8>);
}

pub struct FlUrlUploader {
    pub url: String,
    pub api_key: Option<String>,
    pub product: Option<String>,
    pub seq_debug: bool,
    pub compress: bool,
    pub timeout: Duration,
}

impl FlUrlUploader {
    pub fn new(
        url: String,
        api_key: Option<String>,
        product: Option<String>,
        timeout: Duration,
    ) -> Self {
        Self {
            url,
            api_key,
            product,
            seq_debug: std::env::var("SEQ_DEBUG").is_ok(),
            compress: std::env::var("SEQ_COMPRESS").is_ok(),
            timeout,
        }
    }

    pub fn matches(
        &self,
        url: &str,
        api_key: &Option<String>,
        product: &Option<String>,
        timeout: Duration,
    ) -> bool {
        self.url == url
            && &self.api_key == api_key
            && &self.product == product
            && self.timeout == timeout
    }
}

#[async_trait::async_trait]
impl LogsChunkUploader for FlUrlUploader {
    async fn upload_chunk(&self, chunk_to_upload: Vec<u8>) {
        const MAX_ATTEMPTS: u32 = 4;

        if self.seq_debug {
            if chunk_to_upload.len() > 256 {
                println!("Sending log len={}", chunk_to_upload.len());
            } else {
                println!("Sending log: [{:?}]", std::str::from_utf8(&chunk_to_upload));
            }
        }

        let mut attempt_no: u32 = 0;
        let mut chunk = Some(chunk_to_upload);
        loop {
            attempt_no += 1;

            let data = if attempt_no == MAX_ATTEMPTS {
                chunk.take().unwrap()
            } else {
                chunk.as_ref().unwrap().clone()
            };

            let mut fl_url = FlUrl::new(self.url.as_str())
                .set_timeout(self.timeout)
                .append_path_segment("api")
                .append_path_segment("events")
                .append_path_segment("raw")
                .with_header("Accept", "*/*")
                .with_header("Content-Type", "application/vnd.serilog.clef")
                .with_retries(3);

            if self.compress && data.len() > 1024 * 1024 {
                fl_url = fl_url.compress();
            }

            if let Some(api_key) = self.api_key.as_ref() {
                fl_url = fl_url.with_header("X-Seq-ApiKey", api_key);
            };

            if let Some(product) = self.product.as_ref() {
                fl_url = fl_url.with_header(PRODUCT_SCOPE_HEADER, product);
            };

            let response = fl_url
                .append_query_param("clef", NULL_PARAM)
                .post(HttpRequestBody::Raw {
                    data,
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
                    if attempt_no >= MAX_ATTEMPTS {
                        return;
                    }
                }
                Err(err) => {
                    eprintln!(
                        "Attempt: {} Error while uploading logs to seq. Err: {:?}",
                        attempt_no, err
                    );

                    if attempt_no >= MAX_ATTEMPTS {
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

const MAX_RESPONSE_BODY_SIZE: usize = 1024 * 1024;

async fn print_fl_url_response(response: &mut FlUrlResponse) {
    println!("Result: {}", response.get_status_code());
    let body = match response.get_body() {
        Ok(body) => body,

        Err(err) => {
            eprintln!("Error while reading response body. Err: {:?}", err);
            return;
        }
    };

    let body = match body.into_vec(MAX_RESPONSE_BODY_SIZE).await {
        Ok(body) => body,

        Err(err) => {
            eprintln!("Error while reading response body. Err: {:?}", err);
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

#[cfg(test)]
mod tests {
    use std::{
        io::{Read, Write},
        net::TcpListener,
        sync::mpsc,
        time::Duration,
    };

    use super::{FlUrlUploader, LogsChunkUploader};

    // Takes one request, answers 204 to it and gives its headers, the names in lower case
    fn start_server(listener: TcpListener) -> mpsc::Receiver<Vec<(String, String)>> {
        let (sender, receiver) = mpsc::channel();

        std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();

            let mut request = Vec::new();
            let mut buffer = [0u8; 4096];

            let head_len = loop {
                if let Some(pos) = request.windows(4).position(|itm| itm == b"\r\n\r\n") {
                    break pos + 4;
                }

                let read = stream.read(&mut buffer).unwrap();
                assert!(read > 0, "Request is over before its headers are");
                request.extend_from_slice(&buffer[..read]);
            };

            let headers: Vec<(String, String)> = std::str::from_utf8(&request[..head_len])
                .unwrap()
                .lines()
                .skip(1)
                .filter_map(|line| line.split_once(':'))
                .map(|(name, value)| (name.trim().to_lowercase(), value.trim().to_string()))
                .collect();

            let content_length = headers
                .iter()
                .find(|(name, _)| name == "content-length")
                .map(|(_, value)| value.parse::<usize>().unwrap())
                .unwrap_or(0);

            while request.len() < head_len + content_length {
                let read = stream.read(&mut buffer).unwrap();
                assert!(read > 0, "Request is over before its body is");
                request.extend_from_slice(&buffer[..read]);
            }

            stream
                .write_all(b"HTTP/1.1 204 No Content\r\nConnection: close\r\n\r\n")
                .unwrap();

            sender.send(headers).unwrap();
        });

        receiver
    }

    async fn upload_and_get_request_headers(product: Option<&str>) -> Vec<(String, String)> {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        let server = start_server(listener);

        let uploader = FlUrlUploader::new(
            url,
            None,
            product.map(|itm| itm.to_string()),
            Duration::from_secs(5),
        );
        uploader.upload_chunk(b"{}".to_vec()).await;

        server.recv_timeout(Duration::from_secs(5)).unwrap()
    }

    #[tokio::test]
    async fn test_product_is_sent_as_scope_header() {
        let headers = upload_and_get_request_headers(Some("my-product")).await;

        let scope: Vec<&str> = headers
            .iter()
            .filter(|(name, _)| name == "scope")
            .map(|(_, value)| value.as_str())
            .collect();

        assert_eq!(vec!["my-product"], scope);
    }

    #[tokio::test]
    async fn test_no_product_no_scope_header() {
        let headers = upload_and_get_request_headers(None).await;

        assert!(headers.iter().any(|(name, _)| name == "content-type"));
        assert!(headers.iter().all(|(name, _)| name != "scope"));
    }
}
