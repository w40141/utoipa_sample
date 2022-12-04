use chrono::{DateTime, Local};
use serde::Serialize;
use ulid::Ulid;

use crate::domain::user::User;

#[derive(Serialize)]
pub struct RegisteredUserResponse {
    id: Ulid,
    name: String,
    email: String,
    created_at: DateTime<Local>,
}

impl From<User> for RegisteredUserResponse {
    fn from(v: User) -> Self {
        RegisteredUserResponse {
            id: v.id().to_owned(),
            name: v.name().to_owned(),
            email: v.email().to_owned(),
            created_at: v.created_at().to_owned(),
        }
    }
}

#[derive(Serialize)]
pub struct SearchedUserResponse {
    name: String,
    email: String,
    created_at: DateTime<Local>,
}

#[derive(Serialize)]
pub struct PostTweetResponse {
    id: Ulid,
    content: String,
    user_id: u64,
    user_name: String,
    created_at: DateTime<Local>,
}

impl PostTweetResponse {
    pub fn new(user: User, content: String) -> Self {
        Self {
            id: user,
            content,
            user_id,
            user_name: todo!(),
            created_at: todo!(),
        }
    }
}

#[derive(Serialize)]
pub struct GetAllTweetResponse {
    user_id: Ulid,
    tweet: Vec<Tweet>,
}

#[derive(Serialize)]
pub struct Tweet {
    id: Ulid,
    content: String,
    created_at: DateTime<Local>,
}
