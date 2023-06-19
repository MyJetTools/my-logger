use std::{collections::HashMap, sync::Arc};

use crate::MyLoggerReader;

pub struct MyLoggerInner {
    readers: Vec<Arc<dyn MyLoggerReader + Send + Sync + 'static>>,
    populated_params: HashMap<String, String>,
}

impl MyLoggerInner {
    pub fn new() -> Self {
        Self {
            readers: Vec::new(),
            populated_params: HashMap::new(),
        }
    }

    pub fn populate_params(&mut self, key: String, value: String) {
        self.populated_params.insert(key, value);
    }

    pub fn register_reader(&mut self, reader: Arc<dyn MyLoggerReader + Send + Sync + 'static>) {
        self.readers.push(reader);
    }

    pub fn get_readers(
        &self,
    ) -> impl Iterator<Item = &Arc<dyn MyLoggerReader + Send + Sync + 'static>> {
        self.readers.iter()
    }

    pub fn get_populated_params(&self) -> &HashMap<String, String> {
        &self.populated_params
    }
}
