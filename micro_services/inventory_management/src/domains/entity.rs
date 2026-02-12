use dto::{FlowerDTO, ToolDTO};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Flower {
    pub building_location_number: u32,

    #[serde(flatten)]
    pub flower_dto: FlowerDTO,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tool {
    pub building_location_number: u32,

    #[serde(flatten)]
    pub tool_dto: ToolDTO,
}
