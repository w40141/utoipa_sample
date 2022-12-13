use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, IntoParams)]
pub struct Name {
    /// User name
    name: String,
}

impl Name {
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Deserialize, ToSchema)]
pub struct RegisterUserRequest {
    name: String,
    email: String,
}

impl RegisterUserRequest {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn email(&self) -> &str {
        &self.email
    }
}

#[derive(Deserialize, ToSchema)]
pub struct PostTweetRequest {
    name: String,
    content: String,
}

impl PostTweetRequest {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}
