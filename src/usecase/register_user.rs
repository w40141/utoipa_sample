use crate::domain::user::User;
use crate::domain::UserRepository;

use super::RegisterUserUsecase;

pub struct RegisterUserHandler {
    handler: Box<dyn UserRepository>,
}

impl RegisterUserHandler {
    pub fn new(handler: Box<dyn UserRepository>) -> Self {
        Self { handler }
    }
}

impl RegisterUserUsecase for RegisterUserHandler {
    fn handle(&self, name: &str, email: &str) -> User {
        self.handler.register_user(name, email)
    }
}
