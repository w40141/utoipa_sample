use std::collections::HashMap;

use crate::domain::user::User;
use crate::domain::UserRepository;

pub struct UserDB;

impl UserRepository for UserDB {
    fn search_user_by(&self, name: &str) -> Option<User> {
        let mut users = HashMap::new();
        users.insert("taro", "taro@example.com");
        users.insert("hanako", "hanako@example.com");
        users.insert("jioo", "jiro@example.com");
        users.insert("satomi", "satomi@example.com");
        let Some(&email) = users.get(name) else {return None};
        Some(User::new(name, email))
    }
}
