use dto::FlowerDTO;

use crate::domains::InventoryRepositoryWrite;
use crate::result::{Error, Result};

pub(crate) struct InventoryUseCases<A>
where
    A: InventoryRepositoryWrite,
{
    adapter: A,
}

impl<A> InventoryUseCases<A>
where
    A: InventoryRepositoryWrite,
{
    pub fn new(adapter: A) -> Self {
        Self { adapter }
    }

    /// Reserve `quantity` flowers of the given `kind` if available.
    /// Returns Ok(()) on success, Err(Error) on failure.
    pub fn reserve_flowers(&mut self, kind: &str, quantity: u32) -> Result<()> {
        if let Some(flower) = self.adapter.get_flower_by_kind(kind) {
            if flower.quantity_left >= quantity {
                let change = -(quantity as i32);
                self.adapter.update_flower_quantity(kind, change);
                Ok(())
            } else {
                Err(Error::CustomInventory("Not enough stock".into()))
            }
        } else {
            Err(Error::CustomInventory("Flower not found".into()))
        }
    }

    /// Helper to get current quantity in tests or callers
    pub fn get_flower_quantity(&self, kind: &str) -> Result<Option<u32>> {
        Ok(self
            .adapter
            .get_flower_by_kind(kind)
            .map(|f| f.quantity_left))
    }

    /// Tool related use-case example (stub)
    pub fn reserve_tool(&mut self, _kind: &str, _quantity: u32) -> Result<()> {
        // Similar pattern as flowers
        Err(Error::CustomInventory("Not implemented".into()))
    }
}
