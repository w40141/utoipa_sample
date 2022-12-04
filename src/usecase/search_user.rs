use ulid::Ulid;

use crate::domain::user::{User, UserRepository};

impl<Ulid> UserRepository<Ulid> for User {
    fn search_user_by(param: Ulid) -> User {
        todo!()
    }
}
