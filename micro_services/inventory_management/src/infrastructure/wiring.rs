use crate::{
    adapters::MongodbInventoryRepository, application::InventoryApplication,
    domains::InventoryRepositoryWrite, result::Result,
};

const DEFAULT_INVENTORY_DB_URI: &str = "mongodb://localhost:27017";

pub async fn build_inventory_application()
-> Result<InventoryApplication<impl InventoryRepositoryWrite + Send>> {
    let db_path =
        std::env::var("INVENTORY_DB_URI").unwrap_or_else(|_| DEFAULT_INVENTORY_DB_URI.to_string());

    let repo = MongodbInventoryRepository::new_read_write(&db_path).await?;

    Ok(InventoryApplication::new(repo))
}
