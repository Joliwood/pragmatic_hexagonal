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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    use crate::domains::InventoryRepositoryRead;

    // Simple in-memory repository to test UseCases
    struct InMemoryRepo {
        flowers: HashMap<String, FlowerDTO>,
    }

    impl InMemoryRepo {
        fn new() -> Self {
            Self {
                flowers: HashMap::new(),
            }
        }
    }

    impl InventoryRepositoryRead for InMemoryRepo {
        fn get_flower_by_kind(&self, kind: &str) -> Option<FlowerDTO> {
            self.flowers.get(kind).cloned()
        }

        fn get_tool_by_kind(&self, _kind: &str) -> Option<crate::domains::Tool> {
            None
        }
    }

    impl crate::domains::InventoryRepositoryWrite for InMemoryRepo {
        fn update_flower_quantity(&mut self, kind: &str, quantity_change: i32) {
            if let Some(f) = self.flowers.get_mut(kind) {
                let new = (f.quantity_left as i32 + quantity_change) as u32;
                f.quantity_left = new;
            }
        }

        fn update_tool_quantity(&mut self, _kind: &str, _quantity_change: i32) {
            // not needed for tests
        }
    }

    #[test]
    fn reserve_flowers_success() {
        let mut repo = InMemoryRepo::new();
        repo.flowers.insert(
            "Rose".to_string(),
            FlowerDTO {
                kind: "Rose".to_string(),
                quantity_left: 10,
            },
        );

        let mut uc = InventoryUseCases::new(repo);
        assert!(uc.reserve_flowers("Rose", 3).is_ok());

        match uc.get_flower_quantity("Rose") {
            Ok(Some(q)) => assert_eq!(q, 7),
            Ok(None) => panic!("Expected Some quantity for Rose"),
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }

    #[test]
    fn reserve_flowers_insufficient() {
        let mut repo = InMemoryRepo::new();
        repo.flowers.insert(
            "Tulip".to_string(),
            FlowerDTO {
                kind: "Tulip".to_string(),
                quantity_left: 2,
            },
        );

        let mut uc = InventoryUseCases::new(repo);
        let res = uc.reserve_flowers("Tulip", 5);
        assert!(res.is_err());

        match uc.get_flower_quantity("Tulip") {
            Ok(Some(q)) => assert_eq!(q, 2),
            Ok(None) => panic!("Expected Some quantity for Tulip"),
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }

    #[test]
    fn reserve_flowers_not_found() {
        let repo = InMemoryRepo::new();
        let mut uc = InventoryUseCases::new(repo);
        let res = uc.reserve_flowers("Orchid", 1);
        assert!(res.is_err());
    }
}
