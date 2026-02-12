use mongodb::{Client, error::Result};

use crate::domains::{Flower, InventoryRepositoryWrite, Tool};

pub(crate) struct InventoryServices<A>
where
    A: InventoryRepositoryWrite,
{
    adapter: A,
}

impl<A> InventoryServices<A>
where
    A: InventoryRepositoryWrite,
{
    pub fn new(adapter: A) -> Self {
        Self { adapter }
    }

    pub fn get_flower_by_kind(&self, _kind: &str) -> Option<Flower> {
        // Port method (find_one)
        todo!()
    }

    pub fn update_flower_quantity(&mut self, _kind: &str, _quantity_change: i32) {
        // Port method (update_one)
        todo!()
    }

    pub fn get_tool_by_kind(&self, _kind: &str) -> Option<Tool> {
        // Port method (find_one)
        todo!()
    }

    pub fn update_tool_quantity(&mut self, _kind: &str, _quantity_change: i32) {
        // Port method (update_one)
        todo!()
    }
}
