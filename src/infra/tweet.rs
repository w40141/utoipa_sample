use crate::domain::tweet::{Tweet, TweetRepository, Tweets};

pub struct TweetDB;

impl TweetRepository for TweetDB {
    fn get_all_tweet_by(&self, name: String) -> Option<Tweets> {
        let taros_tweets = vec![
            Tweet::new("taro".to_string(), "おはよう".to_string()),
            Tweet::new("taro".to_string(), "おやすみ".to_string()),
            Tweet::new("taro".to_string(), "きょうもげんき".to_string()),
        ];
        let hanakos_tweets = vec![
            Tweet::new("hanako".to_string(), "おはようございます".to_string()),
            Tweet::new("hanako".to_string(), "おやすみなさい".to_string()),
            Tweet::new("hanako".to_string(), "きょうもげんきです".to_string()),
        ];
        let jiros_tweets = vec![Tweet::new("jiro".to_string(), "おなかすいた".to_string())];
        match &*name {
            "taro" => Some(Tweets::new(taros_tweets)),
            "hanako" => Some(Tweets::new(hanakos_tweets)),
            "jiro" => Some(Tweets::new(jiros_tweets)),
            _ => None,
        }
    }
}
