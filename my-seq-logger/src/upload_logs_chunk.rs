use std::sync::Arc;

use my_logger_core::{MyLogEvent, PopulatedParams};

use crate::LogsChunkUploader;

const MAX_CHUNK_SIZE: usize = 1024 * 1024 * 2;
const INITIAL_CHUNK_CAPACITY: usize = 64 * 1024;
const INITIAL_PAYLOAD_CAPACITY: usize = 1024;

pub async fn upload_log_events_chunk(
    uploader: &impl LogsChunkUploader,
    populated_params: PopulatedParams,
    data: Vec<Arc<MyLogEvent>>,
) {
    let mut chunk_to_upload: Vec<u8> = Vec::with_capacity(INITIAL_CHUNK_CAPACITY);
    let mut payload = String::with_capacity(INITIAL_PAYLOAD_CAPACITY);

    for log_event in data.iter() {
        payload = super::serialize(payload, log_event, &populated_params);

        let separator_len = if chunk_to_upload.is_empty() { 0 } else { 2 };
        let projected = chunk_to_upload.len() + separator_len + payload.len();

        if projected > MAX_CHUNK_SIZE && !chunk_to_upload.is_empty() {
            let cap = chunk_to_upload.capacity();
            let to_send = std::mem::replace(&mut chunk_to_upload, Vec::with_capacity(cap));
            uploader.upload_chunk(to_send).await;
        }

        if !chunk_to_upload.is_empty() {
            chunk_to_upload.extend_from_slice(b"\r\n");
        }
        chunk_to_upload.extend_from_slice(payload.as_bytes());
    }

    if !chunk_to_upload.is_empty() {
        uploader.upload_chunk(chunk_to_upload).await;
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use my_logger_core::{MyLogEvent, PopulatedParams};
    use rust_extensions::date_time::DateTimeAsMicroseconds;

    use crate::LogsChunkUploader;

    pub struct MockUploader;

    #[async_trait::async_trait]
    impl LogsChunkUploader for MockUploader {
        async fn upload_chunk(&self, chunk: Vec<u8>) {
            println!("Uploaded {}", chunk.len());
        }
    }

    #[tokio::test]
    async fn test_chunk_compiler() {
        let events = vec![
            Arc::new(MyLogEvent {
                dt: DateTimeAsMicroseconds::now(),
                level: my_logger_core::LogLevel::Info,
                process: "Test".to_string(),
                message: "Message".to_string(),
                context: None,
            }),
            Arc::new(MyLogEvent {
                dt: DateTimeAsMicroseconds::now(),
                level: my_logger_core::LogLevel::Info,
                process: "Test".to_string(),
                message: "Message2".to_string(),
                context: None,
            }),
        ];

        let mock_uploader = MockUploader;

        super::upload_log_events_chunk(&mock_uploader, PopulatedParams::new_empty(), events).await;
    }
}
