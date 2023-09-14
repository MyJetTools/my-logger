use std::sync::atomic::AtomicBool;

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
            print_fatal_errors: AtomicBool::new(true),
            print_errors: AtomicBool::new(true),
            print_warnings: AtomicBool::new(true),
            print_infos: AtomicBool::new(true),
            print_debug: AtomicBool::new(true),
        }
    }

    pub fn set_print_fatal_errors(&self, value: bool) {
        self.print_fatal_errors
            .store(value, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn set_print_errors(&self, value: bool) {
        self.print_errors
            .store(value, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn set_print_warnings(&self, value: bool) {
        self.print_warnings
            .store(value, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn set_print_infos(&self, value: bool) {
        self.print_infos
            .store(value, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn set_print_debug(&self, value: bool) {
        self.print_debug
            .store(value, std::sync::atomic::Ordering::SeqCst);
    }
}
