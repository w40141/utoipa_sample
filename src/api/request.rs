use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisteredUserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct PostTweetRequest {
    pub name: String,
    pub content: String,
}
