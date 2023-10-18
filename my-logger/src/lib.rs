extern crate my_logger_core;

pub use my_logger_core::*;
#[macro_export]
macro_rules! log_debug {
    (
        $process:expr,
        $message:expr,
        $( $key:tt = $value:expr ),*
    ) => {
        let mut logger_context = my_logger::LogEventCtx::new();
        $(
            let value = format!("{:?}", $value);
            logger_context = logger_context.add($key, value);
        )*

        my_logger::LOGGER.write_log(
            my_logger::LogLevel::Debug,
            $process.to_string(),
            $message.to_string(),
            logger_context.get_result()
        );
    };
}

#[macro_export]
macro_rules! log_err {
    (
        $process:expr,
        $message:expr,
        $( $key:tt = $value:expr ),*
    ) => {
        let mut logger_context = my_logger::LogEventCtx::new();
        $(
            let value = format!("{:?}", $value);
            logger_context = logger_context.add($key, value);
        )*

        my_logger::LOGGER.write_log(
            my_logger::LogLevel::Error,
            $process.to_string(),
            $message.to_string(),
            logger_context.get_result()
        );
    };
}

#[macro_export]
macro_rules! log_warning {
    (
        $process:expr,
        $message:expr,
        $( $key:tt = $value:expr ),*
    ) => {
        let mut logger_context = my_logger::LogEventCtx::new();
        $(
            let value = format!("{:?}", $value);
            logger_context = logger_context.add($key, value);
        )*

        my_logger::LOGGER.write_log(
            my_logger::LogLevel::Warning,
            $process.to_string(),
            $message.to_string(),
            logger_context.get_result()
        );
    };
}

#[macro_export]
macro_rules! log_fatal {
    (
        $process:expr,
        $message:expr,
        $( $key:tt = $value:expr ),*
    ) => {
        let mut logger_context = my_logger::LogEventCtx::new();
        $(
            let value = format!("{:?}", $value);
            logger_context = logger_context.add($key, value);
        )*

        my_logger::LOGGER.write_log(
            my_logger::LogLevel::FatalError,
            $process.to_string(),
            $message.to_string(),
            logger_context.get_result()
        );
    };
}

#[macro_export]
macro_rules! log_info {
    (
        $process:expr,
        $message:expr,
        $( $key:tt = $value:expr ),*
    ) => {
        let mut logger_context = my_logger::LogEventCtx::new();
        $(
            let value = format!("{:?}", $value);
            logger_context = logger_context.add($key, value);
        )*

        my_logger::LOGGER.write_log(
            my_logger::LogLevel::Info,
            $process.to_string(),
            $message.to_string(),
            logger_context.get_result()
        );
    };
}

#[cfg(feature = "my-seq-logger")]
pub extern crate my_seq_logger;
