use crate::domains::User;

/*
 * Here the repository will only have the functions needed
 * to interact with its own DB.
 *
 * - Flower > inventory
 * - Tool > inventory
 * - User
 */

pub trait ClientRepositoryRead {
    fn get_user_by_mail(&self, mail: &str) -> Option<User>;
}

/// The WRITE trait inherits methods from the READ trait
pub(crate) trait ClientRepositoryWrite: ClientRepositoryRead {
    fn create_user(&mut self, mail: &str, address: &str, name: &str);
    fn get_user_by_id(&self, id: &i32) -> Option<User>;
}
