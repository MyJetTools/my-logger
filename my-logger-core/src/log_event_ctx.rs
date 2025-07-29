use std::collections::{BTreeMap, HashMap};

use rust_extensions::StrOrString;

#[derive(Debug, Clone)]
pub struct LogEventCtx(Option<HashMap<String, String>>);

impl LogEventCtx {
    pub fn new() -> Self {
        Self(None)
    }

    pub fn from_object(key: impl Into<StrOrString<'static>>, value: &impl std::fmt::Debug) -> Self {
        let result = Self::new();
        result.add_object(key, value)
    }

    pub fn add<'s>(
        mut self,
        key: impl Into<StrOrString<'static>>,
        value: impl Into<StrOrString<'s>>,
    ) -> Self {
        if self.0.is_none() {
            self.0 = Some(HashMap::new());
        }

        let key = key.into();
        let value = value.into();
        self.0
            .as_mut()
            .unwrap()
            .insert(key.to_string(), value.to_string());
        self
    }

    pub fn add_object(
        self,
        key: impl Into<StrOrString<'static>>,
        value: &impl std::fmt::Debug,
    ) -> Self {
        self.add(key, format!("{:?}", value))
    }

    pub fn get_result(self) -> Option<HashMap<String, String>> {
        self.0
    }
}

impl Into<LogEventCtx> for Option<LogEventCtx> {
    fn into(self) -> LogEventCtx {
        self.unwrap_or_else(LogEventCtx::new)
    }
}

impl Into<LogEventCtx> for HashMap<String, String> {
    fn into(self) -> LogEventCtx {
        LogEventCtx(Some(self))
    }
}

impl Into<LogEventCtx> for BTreeMap<String, String> {
    fn into(self) -> LogEventCtx {
        LogEventCtx(Some(self.into_iter().collect()))
    }
}
