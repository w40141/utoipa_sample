use crate::domain::TweetRepository;
use crate::domain::tweet::Tweets;

use anyhow::{anyhow, Result};

use super::GetAllTweetsUsecase;

pub struct GetAllTweetsHandler {
    handler: Box<dyn TweetRepository>,
}

impl GetAllTweetsHandler {
    pub fn new(handler: Box<dyn TweetRepository>) -> Self {
        Self { handler }
    }
}

impl GetAllTweetsUsecase for GetAllTweetsHandler {
    fn handle(&self, user_name: String) -> Result<Tweets> {
        let Some(t) = self.handler.get_all_tweet_by(user_name) else {return Err(anyhow!("User name is not found."))};
        Ok(t)
    }
}
