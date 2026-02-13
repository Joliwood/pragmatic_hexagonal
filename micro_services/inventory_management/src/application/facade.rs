use crate::application::InventoryUseCases;
use crate::domains::InventoryRepositoryWrite;
use crate::result::Result;

/// Unique entry point for external microservice usages.
///
/// Internally uses an mongodb repository for demo/teaching purposes (no network/DB)
/// but at an external utilisation, we only see application facade with use-cases, with
/// no knowledge of what DB kind we are using.
///
/// The use-cases can use write or read-only access, it doesn't matter here.
pub struct InventoryApplication<R>
where
    R: InventoryRepositoryWrite,
{
    usecases: InventoryUseCases<R>,
}

impl<R> InventoryApplication<R>
where
    R: InventoryRepositoryWrite,
{
    /// Create a new application facade from an inventory repository port.
    pub fn new(repo: R) -> Self {
        Self {
            usecases: InventoryUseCases::new(repo),
        }
    }

    pub fn get_flower_quantity(&self, kind: &str) -> Result<Option<u32>> {
        self.usecases.get_flower_quantity(kind)
    }

    pub fn reserve_flowers(&mut self, kind: &str, quantity: u32) -> Result<()> {
        self.usecases.reserve_flowers(kind, quantity)
    }
}
