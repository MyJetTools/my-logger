use std::sync::Arc;

use rust_extensions::StrOrString;

use crate::{MyLoggerReader, PopulatedParams};

pub struct LogReaders {
    readers: Vec<Arc<dyn MyLoggerReader + Send + Sync + 'static>>,
    populated_params: PopulatedParams,
}

impl LogReaders {
    pub fn new(populated_params: Vec<(&'static str, StrOrString<'static>)>) -> Self {
        Self {
            readers: Vec::new(),
            populated_params: PopulatedParams::new(populated_params),
        }
    }

    fn clone(&self) -> Self {
        LogReaders {
            readers: self.readers.clone(),
            populated_params: self.populated_params.clone(),
        }
    }

    pub fn populate_params(
        &self,
        params: impl Iterator<Item = (&'static str, StrOrString<'static>)>,
    ) -> Self {
        let mut result = self.clone();
        for param in params {
            result.populated_params.push(param.0, param.1);
        }

        result
    }

    pub fn register_reader(&self, reader: Arc<dyn MyLoggerReader + Send + Sync + 'static>) -> Self {
        let mut result = self.clone();
        result.readers.push(reader);
        result
    }

    pub fn get_readers(&self) -> &[Arc<dyn MyLoggerReader + Send + Sync + 'static>] {
        self.readers.as_slice()
    }

    pub fn get_populated_params(&self) -> &PopulatedParams {
        &self.populated_params
    }
}
