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
    fn handle(&self, name: String, email: String) -> User;
}

pub trait PostTweetUsecase {
    fn handle(&self, user_name: String, content: String) -> Result<Tweet>;
}

pub trait SearchUserUsecase {
    fn handle(&self, name: String) -> Result<User>;
}

pub trait GetAllTweetsUsecase {
    fn handle(&self, name: String) -> Result<Tweets>;
}
