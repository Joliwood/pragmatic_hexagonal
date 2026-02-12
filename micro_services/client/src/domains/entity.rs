use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use dto::UserDTO;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub name: String,

    #[serde(flatten)]
    pub user_dto: UserDTO,
}
