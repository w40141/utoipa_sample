use anyhow::{anyhow, Result};

use crate::domain::tweet::Tweet;
use crate::domain::{TweetRepository, UserRepository};

use super::PostTweetUsecase;

pub struct PostTweetHandler {
    tweet_handler: Box<dyn TweetRepository>,
    user_handler: Box<dyn UserRepository>,
}

impl PostTweetHandler {
    pub fn new(
        tweet_handler: Box<dyn TweetRepository>,
        user_handler: Box<dyn UserRepository>,
    ) -> Self {
        Self {
            tweet_handler,
            user_handler,
        }
    }
}

impl PostTweetUsecase for PostTweetHandler {
    fn handle(&self, user_name: &str, content: &str) -> Result<Tweet> {
        let Some(u) = self.user_handler.search_user_by(user_name)
            else {return Err(anyhow!("User name is not found."))};
        Ok(self.tweet_handler.post_tweet(u.name(), content))
    }
}
