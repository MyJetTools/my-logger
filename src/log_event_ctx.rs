use std::collections::HashMap;

use rust_extensions::StrOrString;

pub struct LogEventCtx(Option<HashMap<String, String>>);

impl LogEventCtx {
    pub fn new() -> Self {
        Self(None)
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

    pub fn get_result(self) -> Option<HashMap<String, String>> {
        self.0
    }
}

impl Into<LogEventCtx> for Option<LogEventCtx> {
    fn into(self) -> LogEventCtx {
        self.unwrap_or_else(LogEventCtx::new)
    }
}
