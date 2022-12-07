use anyhow::Result;

use crate::domain::{
    tweet::{Tweet, Tweets},
    user::User,
};

pub mod get_all_tweets;
pub mod post_tweet;
pub mod register_user;
pub mod search_user;

pub trait RegisterUserUsecase {
    fn handle(&self, name: &str, email: &str) -> User;
}

pub trait PostTweetUsecase {
    fn handle(&self, user_name: &str, content: &str) -> Result<Tweet>;
}

pub trait SearchUserUsecase {
    fn handle(&self, name: &str) -> Result<User>;
}

pub trait GetAllTweetsUsecase {
    fn handle(&self, name: &str) -> Result<Tweets>;
}
