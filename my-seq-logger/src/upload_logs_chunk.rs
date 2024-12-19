use std::sync::Arc;

use my_logger_core::{MyLogEvent, PopulatedParams};

use crate::LogsChunkUploader;

const MAX_CHUNK_SIZE: usize = 1024 * 1024 * 2;

pub async fn upload_log_events_chunk(
    uploader: &impl LogsChunkUploader,
    populated_params: PopulatedParams,
    data: Vec<Arc<MyLogEvent>>,
) {
    let mut chunk_to_upload = Vec::new();

    for log_event in data.iter() {
        let payload = super::serialize(log_event, &populated_params);

        let merge_slice =
            chunk_to_upload.len() == 0 || chunk_to_upload.len() + payload.len() < MAX_CHUNK_SIZE;

        if merge_slice {
            if chunk_to_upload.len() > 0 {
                chunk_to_upload.push(13);
                chunk_to_upload.push(10);
            }
            chunk_to_upload.extend(payload);
        } else {
            uploader.upload_chunk(chunk_to_upload.as_slice()).await;
            chunk_to_upload.clear();
            chunk_to_upload.extend(payload);
        }
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
        async fn upload_chunk(&self, chunk: &[u8]) {
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
