use chrono::{DateTime, Local};
use ulid::Ulid;

pub struct Tweet {
    id: Ulid,
    user_id: Ulid,
    content: String,
    created_at: DateTime<Local>,
}

impl Tweet {
    pub fn new(user_id: Ulid, content: String) -> Self {
        Self {
            id: Ulid::new(),
            user_id,
            content,
            created_at: Local::now(),
        }
    }

    pub fn id(&self) -> &Ulid {
        &self.id
    }

    pub fn user_id(&self) -> &Ulid {
        &self.user_id
    }

    pub fn content(&self) -> &String {
        &self.content
    }

    pub fn created_at(&self) -> &DateTime<Local> {
        &self.created_at
    }
}

pub struct Tweets(Vec<Tweet>);

impl Tweets {
    pub fn new(tweets: Vec<Tweet>) -> Self {
        Self(tweets)
    }

    pub fn list(&self) -> &Vec<Tweet> {
        &self.0
    }
}

pub trait TweetRepository<T> {
    fn get_all_tweet_by(param: T) -> Tweets;
}
