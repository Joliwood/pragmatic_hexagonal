use crate::adapters::InMemoryInventoryRepository;
use crate::application::InventoryUseCases;
use crate::result::Result;

/// Unique entry point for external microservice usages.
/// Internally uses an in-memory repository for demo/teaching purposes (no network/DB).
pub struct InventoryApplication {
    usecases: InventoryUseCases<InMemoryInventoryRepository>,
}

impl InventoryApplication {
    /// Create a new application facade with an empty in-memory repository
    pub fn new_inmemory() -> Self {
        let repo = InMemoryInventoryRepository::new();
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
