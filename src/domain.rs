use self::{
    tweet::{Tweet, Tweets},
    user::User,
};

pub mod tweet;
pub mod user;

pub trait TweetRepository {
    fn get_all_tweet_by(&self, name: &str) -> Option<Tweets>;
    fn post_tweet(&self, user_name: &str, content: &str) -> Tweet {
        Tweet::new(user_name, content)
    }
}

pub trait UserRepository {
    fn search_user_by(&self, name: &str) -> Option<User>;
    fn register_user(&self, name: &str, email: &str) -> User {
        User::new(name, email)
    }
}
