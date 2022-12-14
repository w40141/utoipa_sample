use chrono::{DateTime, Local};
use ulid::Ulid;

#[derive(Debug, Clone)]
pub struct Tweet {
    id: Ulid,
    user_name: String,
    content: String,
    created_at: DateTime<Local>,
}

impl Tweet {
    pub fn new(user_name: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            id: Ulid::new(),
            user_name: user_name.into(),
            content: content.into(),
            created_at: Local::now(),
        }
    }

    pub fn id(&self) -> &Ulid {
        &self.id
    }

    pub fn user_name(&self) -> &String {
        &self.user_name
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
