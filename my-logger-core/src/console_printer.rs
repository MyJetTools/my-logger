use std::sync::atomic::AtomicBool;

use crate::{LogLevel, MyLogEvent};

pub struct ConsoleFilter {
    pub print_debug: AtomicBool,
    pub print_fatal_errors: AtomicBool,
    pub print_errors: AtomicBool,
    pub print_warnings: AtomicBool,
    pub print_infos: AtomicBool,
}

impl ConsoleFilter {
    pub fn new() -> Self {
        Self {
            print_fatal_errors: true.into(),
            print_errors: true.into(),
            print_warnings: true.into(),
            print_infos: true.into(),
            print_debug: true.into(),
        }
    }

    pub fn set_print_fatal_errors(&self, value: bool) {
        self.print_fatal_errors
            .store(value, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn set_print_errors(&self, value: bool) {
        self.print_errors
            .store(value, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn set_print_warnings(&self, value: bool) {
        self.print_warnings
            .store(value, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn set_print_infos(&self, value: bool) {
        self.print_infos
            .store(value, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn set_print_debug(&self, value: bool) {
        self.print_debug
            .store(value, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn print_to_console(&self, log_event: &MyLogEvent) {
        match log_event.level {
            LogLevel::Info => {
                if self.print_infos.load(std::sync::atomic::Ordering::Relaxed) {
                    write_log(log_event);
                }
            }
            LogLevel::Warning => {
                if self
                    .print_warnings
                    .load(std::sync::atomic::Ordering::Relaxed)
                {
                    write_log(&log_event);
                }
            }
            LogLevel::Error => {
                if self.print_errors.load(std::sync::atomic::Ordering::Relaxed) {
                    write_log(&log_event);
                }
            }
            LogLevel::FatalError => {
                if self
                    .print_fatal_errors
                    .load(std::sync::atomic::Ordering::Relaxed)
                {
                    write_log(&log_event);
                }
            }
            LogLevel::Debug => {
                if self.print_debug.load(std::sync::atomic::Ordering::Relaxed) {
                    write_log(log_event);
                }
            }
        }
    }
}

fn write_log(log_event: &MyLogEvent) {
    use std::fmt::Write as FmtWrite;
    use std::io::Write as IoWrite;

    let estimated = 64 + log_event.process.len() + log_event.message.len();
    let mut buf = String::with_capacity(estimated);

    let _ = writeln!(
        &mut buf,
        "{} {}",
        log_event.dt.to_rfc3339(),
        log_event.level.as_str()
    );
    let _ = writeln!(&mut buf, "Process: {}", log_event.process);
    let _ = writeln!(&mut buf, "Message: {}", log_event.message);
    if let Some(ctx) = &log_event.context {
        let _ = writeln!(&mut buf, "Context: {:?}", ctx);
    }
    buf.push_str("-------------------\n");

    let is_err = matches!(log_event.level, LogLevel::Error | LogLevel::FatalError);
    if is_err {
        let stderr = std::io::stderr();
        let _ = stderr.lock().write_all(buf.as_bytes());
    } else {
        let stdout = std::io::stdout();
        let _ = stdout.lock().write_all(buf.as_bytes());
    }
}
