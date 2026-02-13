use dto::FlowerDTO;

use crate::domains::{Flower, Tool};

pub trait InventoryRepositoryRead {
    fn get_flower_by_kind(&self, kind: &str) -> Option<FlowerDTO>;
    fn get_tool_by_kind(&self, kind: &str) -> Option<Tool>;
}

/// The WRITE trait inherits methods from the READ trait
pub trait InventoryRepositoryWrite: InventoryRepositoryRead {
    fn update_flower_quantity(&mut self, kind: &str, quantity_change: i32);
    fn update_tool_quantity(&mut self, kind: &str, quantity_change: i32);
}
