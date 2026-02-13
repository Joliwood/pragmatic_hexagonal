use inventory_management::{build_inventory_application, init_logger};

#[tokio::main]
async fn main() {
    init_logger();

    let Ok(inventory_app) = build_inventory_application().await else {
        println!("Error building inventory application");
        return;
    };

    let _roses = inventory_app.get_flower_quantity("Rose");
}
