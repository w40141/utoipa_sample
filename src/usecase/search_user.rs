use anyhow::{anyhow, Result};

use crate::domain::user::{User, UserRepository};

use super::SearchUser;

pub struct SearchUserHandle {
    handler: Box<dyn UserRepository>,
}

impl SearchUserHandle {
    pub fn new(handler: Box<dyn UserRepository>) -> Self {
        Self { handler }
    }
}

impl SearchUser for SearchUserHandle {
    fn handle(&self, user_name: String) -> Result<User> {
        let Some(u) = self.handler.search_user_by(user_name) else {return Err(anyhow!("User name is not found."))};
        Ok(u.clone())
    }
}
