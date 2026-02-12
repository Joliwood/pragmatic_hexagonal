use dto::FlowerDTO;
use mongodb::{Client, error::Result};

use crate::domains::{Flower, InventoryRepositoryRead, InventoryRepositoryWrite, Tool};

pub(crate) struct MongodbInventoryRepository {
    client: Client,
}
impl MongodbInventoryRepository {
    /// External function, read-only access
    pub async fn new_read(db_path: &str) -> Result<Self> {
        let client = Client::with_uri_str(db_path).await?;
        Ok(Self { client })
    }

    /// For the micro-service only, read + write access
    pub(crate) async fn new_read_write(db_path: &str) -> Result<Self> {
        let client = Client::with_uri_str(db_path).await?;
        Ok(Self { client })
    }
}

// Implementations for business needs
impl InventoryRepositoryRead for MongodbInventoryRepository {
    fn get_flower_by_kind(&self, _kind: &str) -> Option<FlowerDTO> {
        // MongoDB logic (find_one)
        todo!()
    }

    fn get_tool_by_kind(&self, _kind: &str) -> Option<Tool> {
        // MongoDB logic (find_one)
        todo!()
    }
}

impl InventoryRepositoryWrite for MongodbInventoryRepository {
    fn update_flower_quantity(&mut self, _kind: &str, _quantity_change: i32) {
        // MongoDB logic (update_one)
        todo!()
    }

    fn update_tool_quantity(&mut self, _kind: &str, _quantity_change: i32) {
        // MongoDB logic (update_one)
        todo!()
    }
}
