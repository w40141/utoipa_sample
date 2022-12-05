use crate::domain::user::User;
use crate::domain::UserRepository;

pub struct UserDB;

impl UserRepository for UserDB {
    fn search_user_by(&self, name: String) -> Option<User> {
        let users = [
            User::new("taro".to_string(), "taro@example.com".to_string()),
            User::new("hanako".to_string(), "hanako@example.com".to_string()),
            User::new("jioo".to_string(), "jiro@example.com".to_string()),
            User::new("satomi".to_string(), "satomi@example.com".to_string()),
        ];
        let Some(u) = users.iter().find(|u| u.name().eq(&name)) else {return None};
        Some(u.clone())
    }
}
