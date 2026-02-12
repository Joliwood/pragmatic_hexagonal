use logger::Logger;

use crate::result::Result;

pub fn init_logger() -> Logger {
    // Custom actions for the client micro-service only

    Logger::default()
}
