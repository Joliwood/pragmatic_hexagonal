use inventory_management::InventoryApplication;

use crate::{
    adapters::MongodbClientRepository, domains::ClientService, infrastructure::init_logger,
};

pub mod adapters;
mod domains;
mod infrastructure;

pub mod result;

#[tokio::main]
async fn main() {
    init_logger();

    // For this demo we use an in-memory inventory application provided by the inventory micro-service.
    // This shows calling the service application (use-cases) directly without touching DB adapters.
    let mut inventory_app = InventoryApplication::new().await.unwrap();

    // -- Practical case 0 > 1 -- //
    let requested_qty = 15;
    let requested_kind = "Rose";

    let Ok(flower_quantity) = inventory_app.get_flower_quantity(requested_kind) else {
        println!(
            "Error fetching fower quantity for type : {}",
            requested_kind
        );
        return;
    };

    let Some(flower_quantity) = flower_quantity else {
        println!("Flower not found");
        return;
    };

    if flower_quantity < requested_qty {
        println!("Not enough flowers in stock");
        return;
    }

    match inventory_app.reserve_flowers(requested_kind, requested_qty) {
        Ok(_) => println!(
            "Successfully reserved {} {}s",
            requested_qty, requested_kind
        ),
        Err(e) => {
            println!("Error reserving flowers: {:?}", e)
        }
    }

    // Note: we kept the client registration out of this demo to avoid async DB calls.
}
