extern crate my_logger_core;

pub use my_logger_core::*;

#[cfg(feature = "my_seq_logger")]
pub extern crate my_seq_logger;