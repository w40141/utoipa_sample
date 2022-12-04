use ulid::Ulid;

use crate::domain::tweet::{Tweets, TweetRepository };

impl<Ulid> TweetRepository<Ulid> for Tweets {
    pub fn get_all_tweet_by(param: Ulid) -> Tweets {
        todo!()
    }
}
