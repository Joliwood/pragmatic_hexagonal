use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct Logger {
    // Could imagine a config, log level, etc. here
}

impl Logger {
    pub fn log(&self, message: &str) {
        println!("[LOG] {}", message);
    }
}
