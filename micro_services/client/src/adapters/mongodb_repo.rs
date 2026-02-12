use mongodb::{Client, bson::oid::ObjectId, error::Result};

use dto::UserDTO;

use crate::{
    adapters::UserDocument,
    domains::{ClientRepositoryRead, ClientRepositoryWrite, User},
};

pub struct MongodbClientRepository {
    client: Client,
}
impl MongodbClientRepository {
    pub async fn new_read(db_path: &str) -> Result<Self> {
        let client = Client::with_uri_str(db_path).await?;
        Ok(Self { client })
    }

    pub(crate) async fn new_write(db_path: &str) -> Result<Self> {
        let client = Client::with_uri_str(db_path).await?;
        Ok(Self { client })
    }
}

// Implementations for business needs
impl ClientRepositoryWrite for MongodbClientRepository {
    fn create_user(&mut self, mail: &str, address: &str, name: &str) {
        let user_document = UserDocument {
            id: ObjectId::new(),
            user_entity: User {
                user_dto: UserDTO {
                    mail: mail.to_string(),
                    address: address.to_string(),
                },
                name: name.to_string(),
            },
        };

        let user = UserDTO {
            mail: mail.to_string(),
            address: address.to_string(),
        };

        // MongoDB logic (insert)
        todo!()
    }

    fn get_user_by_id(&self, _id: &i32) -> Option<User> {
        // MongoDB logic (find_one)
        todo!()
    }
}

impl ClientRepositoryRead for MongodbClientRepository {
    fn get_user_by_mail(&self, _mail: &str) -> Option<User> {
        // MongoDB logic (find_one)
        todo!()
    }
}
