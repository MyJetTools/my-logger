extern crate my_logger_core;

pub use my_logger_core::*;

#[cfg(feature = "my-seq-logger")]
pub extern crate my_seq_logger;