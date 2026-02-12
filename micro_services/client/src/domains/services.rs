use mongodb::{Client, error::Result};

use crate::domains::{ClientRepositoryWrite, User};

pub(crate) struct ClientService<T>
where
    T: ClientRepositoryWrite,
{
    repository: T,
}

impl<T> ClientService<T>
where
    T: ClientRepositoryWrite,
{
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub fn register_user(&mut self, mail: &str, address: &str, name: &str) {
        // The service calls the repository method as defined in the port
        self.repository.create_user(mail, address, name);
    }
}
