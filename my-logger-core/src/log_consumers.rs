use std::sync::Arc;

use rust_extensions::StrOrString;

use crate::{MyLoggerConsumer, PopulatedParams};

pub struct LogConsumers {
    consumers: Vec<Arc<dyn MyLoggerConsumer + Send + Sync + 'static>>,
    populated_params: PopulatedParams,
}

impl LogConsumers {
    pub fn new(populated_params: Vec<(&'static str, StrOrString<'static>)>) -> Self {
        Self {
            consumers: Vec::new(),
            populated_params: PopulatedParams::new(populated_params),
        }
    }

    fn clone(&self) -> Self {
        LogConsumers {
            consumers: self.consumers.clone(),
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

    pub fn register_consumer(
        &self,
        consumer: Arc<dyn MyLoggerConsumer + Send + Sync + 'static>,
    ) -> Self {
        let mut result = self.clone();
        result.consumers.push(consumer);
        result
    }

    pub fn get_consumers(&self) -> &[Arc<dyn MyLoggerConsumer + Send + Sync + 'static>] {
        self.consumers.as_slice()
    }

    pub fn get_populated_params(&self) -> &PopulatedParams {
        &self.populated_params
    }
}
