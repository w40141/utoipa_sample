use serde::Deserialize;
use utoipa::{ToSchema, IntoParams};

#[derive(Deserialize, IntoParams)]
pub struct NamePath {
    pub name: String
}

#[derive(Deserialize, ToSchema)]
pub struct RegisterUserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Deserialize, ToSchema)]
pub struct PostTweetRequest {
    pub name: String,
    pub content: String,
}
