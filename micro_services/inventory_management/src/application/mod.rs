mod facade;
mod use_cases;

/// Only this application is accesible from external
pub use facade::InventoryApplication;
pub(crate) use use_cases::*;
