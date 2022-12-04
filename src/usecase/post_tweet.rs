use anyhow::{anyhow, Result};

use crate::domain::{
    tweet::{Tweet, TweetRepository},
    user::UserRepository,
};

use super::PostTweet;

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

impl PostTweet for PostTweetHandler {
    fn handle(&self, user_name: String, content: String) -> Result<Tweet> {
        let Some(u) = self.user_handler.search_user_by(user_name) else {return Err(anyhow!("User name is not found."))};
        Ok(self.tweet_handler.post_tweet(u.name().to_owned(), content))
    }
}
