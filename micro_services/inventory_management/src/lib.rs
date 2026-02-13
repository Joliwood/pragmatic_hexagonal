mod adapters;
mod application;
mod domains;
mod infrastructure;
mod result;

pub(crate) use adapters::*;
pub use application::InventoryApplication;
pub(crate) use domains::*;
pub use infrastructure::{build_inventory_application, init_logger};
pub use result::{Error, Result};
