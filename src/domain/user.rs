use chrono::{DateTime, Local};
use ulid::Ulid;

#[derive(Debug, Clone)]
pub struct User {
    id: Ulid,
    name: String,
    email: String,
    created_at: DateTime<Local>,
}

impl User {
    pub fn new(name: String, email: String) -> Self {
        Self {
            id: Ulid::new(),
            name,
            email,
            created_at: Local::now(),
        }
    }

    pub fn id(&self) -> &Ulid {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn email(&self) -> &String {
        &self.email
    }

    pub fn created_at(&self) -> &DateTime<Local> {
        &self.created_at
    }
}

pub trait UserRepository {
    fn search_user_by(&self, name: String) -> Option<User>;
    fn register_user(&self, name: String, email: String) -> User {
        User::new(name, email)
    }
}
