pub mod adapters;
pub mod domains;
pub mod infrastructure;
pub mod result;

pub use domains::*;
pub use infrastructure::*;

use crate::adapters::MongodbInventoryRepository;

#[tokio::main]
async fn main() {
    init_logger();

    let mut mongodb_repo = MongodbInventoryRepository::new_read_write("mongodb://localhost:27017")
        .await
        .unwrap();

    let roses = mongodb_repo.get_flower_by_kind("Rose");
}
