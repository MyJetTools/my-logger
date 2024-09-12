use std::{collections::HashMap, sync::Arc};

use crate::MyLoggerReader;

pub struct LogReaders {
    readers: Vec<Arc<dyn MyLoggerReader + Send + Sync + 'static>>,
    populated_params: HashMap<String, String>,
}

impl LogReaders {
    pub fn new(populated_params: HashMap<String, String>) -> Self {
        Self {
            readers: Vec::new(),
            populated_params,
        }
    }

    pub fn populate_params(&mut self, key: String, value: String) {
        self.populated_params.insert(key, value);
    }

    pub fn register_reader(&mut self, reader: Arc<dyn MyLoggerReader + Send + Sync + 'static>) {
        self.readers.push(reader);
    }

    pub fn get_readers(&self) -> &[Arc<dyn MyLoggerReader + Send + Sync + 'static>] {
        self.readers.as_slice()
    }

    pub fn get_populated_params(&self) -> &HashMap<String, String> {
        &self.populated_params
    }
}
