use std::sync::atomic::AtomicU64;

pub struct LogsStatistics {
    pub debugs: AtomicU64,
    pub fatal_errors: AtomicU64,
    pub errors: AtomicU64,
    pub warnings: AtomicU64,
    pub info: AtomicU64,
}

impl LogsStatistics {
    pub fn new() -> Self {
        Self {
            debugs: AtomicU64::new(0),
            fatal_errors: AtomicU64::new(0),
            errors: AtomicU64::new(0),
            warnings: AtomicU64::new(0),
            info: AtomicU64::new(0),
        }
    }
}
