use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserDTO {
    pub mail: String,
    pub address: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FlowerDTO {
    pub kind: String,
    pub quantity_left: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolDTO {
    pub kind: String,
    pub quantity_left: u32,
}
