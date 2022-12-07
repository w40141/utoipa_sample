use crate::domain::tweet::{Tweet, Tweets};
use crate::domain::TweetRepository;

pub struct TweetDB;

impl TweetRepository for TweetDB {
    fn get_all_tweet_by(&self, name: &str) -> Option<Tweets> {
        let taros_tweets = vec![
            Tweet::new("taro", "おはよう"),
            Tweet::new("taro", "おやすみ"),
            Tweet::new("taro", "きょうもげんき"),
        ];
        let hanakos_tweets = vec![
            Tweet::new("hanako", "おはようございます"),
            Tweet::new("hanako", "おやすみなさい"),
            Tweet::new("hanako", "きょうもげんきです"),
        ];
        let jiros_tweets = vec![Tweet::new("jiro", "おなかすいた")];
        match name {
            "taro" => Some(Tweets::new(taros_tweets)),
            "hanako" => Some(Tweets::new(hanakos_tweets)),
            "jiro" => Some(Tweets::new(jiros_tweets)),
            _ => None,
        }
    }
}
