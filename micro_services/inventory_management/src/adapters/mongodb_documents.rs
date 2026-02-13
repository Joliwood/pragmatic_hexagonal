use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use dto::UserDTO;

use crate::{Flower, Tool};

#[derive(Serialize, Deserialize)]
pub struct FlowerDocument {
    pub id: ObjectId,

    #[serde(flatten)]
    pub flower_entity: Flower,
}

#[derive(Serialize, Deserialize)]
pub struct ToolDocument {
    pub id: ObjectId,

    #[serde(flatten)]
    pub tool_entity: Tool,
}
