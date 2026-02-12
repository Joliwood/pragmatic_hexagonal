use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use dto::UserDTO;

use crate::domains::User;

#[derive(Serialize, Deserialize)]
pub struct UserDocument {
    /// Only the service with write access to its DB is supposed to know
    /// the DB type (MongoDB here)
    pub(crate) id: ObjectId,

    /// To inline User fields > https://serde.rs/attr-flatten.html
    #[serde(flatten)]
    pub user_entity: User,
}

impl UserDocument {
    pub fn new(id: ObjectId, mail: String, address: String, name: String) -> Self {
        Self {
            id: ObjectId::new(),
            user_entity: User {
                user_dto: UserDTO {
                    mail: mail.to_string(),
                    address: address.to_string(),
                },
                name: name.to_string(),
            },
        }
    }
}
