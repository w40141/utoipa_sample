use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisteredUserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct SearchedUserRequest {
    pub id: u64,
}

#[derive(Deserialize)]
pub struct PostTweetRequest {
    pub user_id: u64,
    pub content: String,
}

#[derive(Deserialize)]
pub struct GetAllTweetRequest {
    pub user_id: u64,
}
