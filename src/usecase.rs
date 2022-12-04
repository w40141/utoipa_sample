use anyhow::Result;

use crate::domain::{
    tweet::{Tweet, Tweets},
    user::User,
};

pub mod get_all_tweets;
pub mod post_tweet;
pub mod register_user;
pub mod search_user;

pub trait RegisterUser {
    fn handle(&self, name: String, email: String) -> User;
}

pub trait PostTweet {
    fn handle(&self, user_name: String, content: String) -> Result<Tweet>;
}

pub trait SearchUser {
    fn handle(&self, name: String) -> Result<User>;
}

pub trait GetAllTweets {
    fn handle(&self, name: String) -> Result<Tweets>;
}
