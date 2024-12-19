use std::sync::Arc;

use crate::{MyLoggerReader, PopulatedParams};

pub struct LogReaders {
    readers: Vec<Arc<dyn MyLoggerReader + Send + Sync + 'static>>,
    populated_params: PopulatedParams,
}

impl LogReaders {
    pub fn new(populated_params: Vec<(String, String)>) -> Self {
        Self {
            readers: Vec::new(),
            populated_params: PopulatedParams::new(populated_params),
        }
    }

    pub fn populate_params(&mut self, key: String, value: String) {
        self.populated_params.push(key, value);
    }

    pub fn register_reader(&mut self, reader: Arc<dyn MyLoggerReader + Send + Sync + 'static>) {
        self.readers.push(reader);
    }

    pub fn get_readers(&self) -> &[Arc<dyn MyLoggerReader + Send + Sync + 'static>] {
        self.readers.as_slice()
    }

    pub fn get_populated_params(&self) -> &PopulatedParams {
        &self.populated_params
    }
}
