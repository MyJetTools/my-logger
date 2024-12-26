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
    let is_err = match log_event.level {
        LogLevel::Error | LogLevel::FatalError => true,
        _ => false,
    };

    if is_err {
        eprintln!("{} {:?}", log_event.dt.to_rfc3339(), log_event.level);
        eprintln!("Process: {}", log_event.process);
        eprintln!("Message: {}", log_event.message);

        if let Some(ctx) = &log_event.context {
            eprintln!("Context: {:?}", ctx);
        }

        eprintln!("-------------------")
    } else {
        println!("{} {:?}", log_event.dt.to_rfc3339(), log_event.level);
        println!("Process: {}", log_event.process);
        println!("Message: {}", log_event.message);

        if let Some(ctx) = &log_event.context {
            println!("Context: {:?}", ctx);
        }

        println!("-------------------")
    }
}
