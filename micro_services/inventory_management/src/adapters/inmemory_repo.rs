use std::collections::HashMap;
use std::sync::Mutex;

use dto::FlowerDTO;

use crate::domains::{InventoryRepositoryRead, InventoryRepositoryWrite, Tool};

#[derive(Default)]
pub(crate) struct InMemoryInventoryRepository {
    flowers: Mutex<HashMap<String, FlowerDTO>>,
    tools: Mutex<HashMap<String, Tool>>,
}

impl InMemoryInventoryRepository {
    pub fn new() -> Self {
        Self {
            flowers: Mutex::new(HashMap::new()),
            tools: Mutex::new(HashMap::new()),
        }
    }

    /// Helper to seed a flower entry for demos/tests
    pub fn seed_flower(&self, kind: &str, quantity: u32) {
        let mut map = self.flowers.lock().unwrap();
        map.insert(
            kind.to_string(),
            FlowerDTO {
                kind: kind.to_string(),
                quantity_left: quantity,
            },
        );
    }
}

impl InventoryRepositoryRead for InMemoryInventoryRepository {
    fn get_flower_by_kind(&self, kind: &str) -> Option<FlowerDTO> {
        let map = self.flowers.lock().unwrap();
        map.get(kind).cloned()
    }

    fn get_tool_by_kind(&self, kind: &str) -> Option<Tool> {
        let map = self.tools.lock().unwrap();
        map.get(kind).cloned()
    }
}

impl InventoryRepositoryWrite for InMemoryInventoryRepository {
    fn update_flower_quantity(&mut self, kind: &str, quantity_change: i32) {
        let mut map = self.flowers.lock().unwrap();
        if let Some(f) = map.get_mut(kind) {
            let new = ((f.quantity_left as i32) + quantity_change).max(0) as u32;
            f.quantity_left = new;
        }
    }

    fn update_tool_quantity(&mut self, kind: &str, quantity_change: i32) {
        let mut map = self.tools.lock().unwrap();
        if let Some(t) = map.get_mut(kind) {
            let new = ((t.tool_dto.quantity_left as i32) + quantity_change).max(0) as u32;
            t.tool_dto.quantity_left = new;
        }
    }
}
