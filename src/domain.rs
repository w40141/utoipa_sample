use self::{
    tweet::{Tweet, Tweets},
    user::User,
};

pub mod tweet;
pub mod user;

pub trait TweetRepository {
    fn get_all_tweet_by(&self, name: String) -> Option<Tweets>;
    fn post_tweet(&self, user_name: String, content: String) -> Tweet {
        Tweet::new(user_name, content)
    }
}

pub trait UserRepository {
    fn search_user_by(&self, name: String) -> Option<User>;
    fn register_user(&self, name: String, email: String) -> User {
        User::new(name, email)
    }
}
