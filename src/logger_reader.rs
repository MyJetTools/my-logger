use std::{collections::HashMap, sync::Arc};

use super::MyLogEvent;

#[async_trait::async_trait]
pub trait MyLoggerReader {
    async fn write_log(
        &self,
        log_event: Arc<MyLogEvent>,
        populated_params: &HashMap<String, String>,
    );
}
