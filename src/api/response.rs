use chrono::{DateTime, Local};
use serde::Serialize;
use ulid::Ulid;

use crate::domain::{
    tweet::{Tweet, Tweets},
    user::User,
};

#[derive(Serialize)]
pub struct RegisteredUserResponse {
    id: Ulid,
    name: String,
    email: String,
    created_at: DateTime<Local>,
}

impl From<User> for RegisteredUserResponse {
    fn from(v: User) -> Self {
        Self {
            id: v.id().to_owned(),
            name: v.name().to_owned(),
            email: v.email().to_owned(),
            created_at: v.created_at().to_owned(),
        }
    }
}

#[derive(Serialize)]
pub struct PostTweetResponse {
    id: Ulid,
    content: String,
    user_name: String,
    created_at: DateTime<Local>,
}

impl From<Tweet> for PostTweetResponse {
    fn from(v: Tweet) -> Self {
        Self {
            id: v.id().to_owned(),
            content: v.content().to_owned(),
            user_name: v.user_name().to_owned(),
            created_at: v.created_at().to_owned(),
        }
    }
}

impl From<&Tweet> for PostTweetResponse {
    fn from(v: &Tweet) -> Self {
        Self {
            id: v.id().to_owned(),
            content: v.content().to_owned(),
            user_name: v.user_name().to_owned(),
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

impl From<User> for SearchedUserResponse {
    fn from(v: User) -> Self {
        Self {
            name: v.name().to_owned(),
            email: v.email().to_owned(),
            created_at: v.created_at().to_owned(),
        }
    }
}

#[derive(Serialize)]
pub struct GetAllTweetResponse {
    tweets: Vec<PostTweetResponse>,
}

impl From<Tweets> for GetAllTweetResponse {
    fn from(v: Tweets) -> Self {
        Self {
            tweets: v.list().iter().map(|t| PostTweetResponse::from(t)).collect(),
        }
    }
}
