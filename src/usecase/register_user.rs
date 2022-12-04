use crate::domain::user::{User, UserRepository};

use super::RegisterUser;

pub struct RegisterUserHandler {
    handler: Box<dyn UserRepository>,
}

impl RegisterUserHandler {
    pub fn new(handler: Box<dyn UserRepository>) -> Self {
        Self { handler }
    }
}

impl RegisterUser for RegisterUserHandler {
    fn handle(&self, name: String, email: String) -> User {
        self.handler.register_user(name, email)
    }
}
