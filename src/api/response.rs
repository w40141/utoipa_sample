use chrono::{DateTime, Local};
use serde::Serialize;
use utoipa::ToSchema;

use crate::domain::{
    tweet::{Tweet, Tweets},
    user::User,
};

#[derive(Serialize, ToSchema)]
pub struct RegisterUserResponse {
    id: String,
    name: String,
    email: String,
    created_at: DateTime<Local>,
}

impl From<User> for RegisterUserResponse {
    fn from(v: User) -> Self {
        Self {
            id: v.id().to_string(),
            name: v.name().to_string(),
            email: v.email().to_string(),
            created_at: *v.created_at(),
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct PostTweetResponse {
    id: String,
    content: String,
    user_name: String,
    created_at: DateTime<Local>,
}

impl From<Tweet> for PostTweetResponse {
    fn from(v: Tweet) -> Self {
        Self {
            id: v.id().to_string(),
            content: v.content().to_string(),
            user_name: v.user_name().to_string(),
            created_at: *v.created_at(),
        }
    }
}

impl From<&Tweet> for PostTweetResponse {
    fn from(v: &Tweet) -> Self {
        Self::from(v.clone())
    }
}

#[derive(Serialize, ToSchema)]
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

#[derive(Serialize, ToSchema)]
pub struct GetAllTweetResponse {
    tweets: Vec<PostTweetResponse>,
}

impl From<Tweets> for GetAllTweetResponse {
    fn from(v: Tweets) -> Self {
        Self {
            tweets: v.list().iter().map(PostTweetResponse::from).collect(),
        }
    }
}
