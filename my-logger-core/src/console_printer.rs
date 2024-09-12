use rust_extensions::UnsafeValue;

use crate::{LogLevel, MyLogEvent};

pub struct ConsoleFilter {
    pub print_debug: UnsafeValue<bool>,
    pub print_fatal_errors: UnsafeValue<bool>,
    pub print_errors: UnsafeValue<bool>,
    pub print_warnings: UnsafeValue<bool>,
    pub print_infos: UnsafeValue<bool>,
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
        self.print_fatal_errors.set_value(value);
    }

    pub fn set_print_errors(&self, value: bool) {
        self.print_errors.set_value(value);
    }

    pub fn set_print_warnings(&self, value: bool) {
        self.print_warnings.set_value(value);
    }

    pub fn set_print_infos(&self, value: bool) {
        self.print_infos.set_value(value);
    }

    pub fn set_print_debug(&self, value: bool) {
        self.print_debug.set_value(value);
    }

    pub fn print_to_console(&self, log_event: &MyLogEvent) {
        match log_event.level {
            LogLevel::Info => {
                if self.print_infos.get_value() {
                    write_log(log_event);
                }
            }
            LogLevel::Warning => {
                if self.print_warnings.get_value() {
                    write_log(&log_event);
                }
            }
            LogLevel::Error => {
                if self.print_errors.get_value() {
                    write_log(&log_event);
                }
            }
            LogLevel::FatalError => {
                if self.print_fatal_errors.get_value() {
                    write_log(&log_event);
                }
            }
            LogLevel::Debug => {
                if self.print_debug.get_value() {
                    write_log(log_event);
                }
            }
        }
    }
}

fn write_log(log_event: &MyLogEvent) {
    println!("{} {:?}", log_event.dt.to_rfc3339(), log_event.level);
    println!("Process: {}", log_event.process);
    println!("Message: {}", log_event.message);

    if let Some(ctx) = &log_event.context {
        println!("Context: {:?}", ctx);
    }

    println!("-------------------")
}
