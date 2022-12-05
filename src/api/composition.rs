use crate::infra::tweet::TweetDB;
use crate::infra::user::UserDB;
use crate::usecase::get_all_tweets::GetAllTweetsHandler;
use crate::usecase::post_tweet::PostTweetHandler;
use crate::usecase::register_user::RegisterUserHandler;
use crate::usecase::search_user::SearchUserHandle;

use super::execute::{
    GetAllTweetsExecute, PostTweetExecute, RegisterUserExecute, SearchUserEcecute,
};

pub struct Composition;

impl Composition {
    pub fn register_user() -> RegisterUserExecute {
        RegisterUserExecute::new(Box::new(RegisterUserHandler::new(Box::new(UserDB))))
    }

    pub fn post_tweet() -> PostTweetExecute {
        PostTweetExecute::new(Box::new(PostTweetHandler::new(
            Box::new(TweetDB),
            Box::new(UserDB),
        )))
    }

    pub fn search_user() -> SearchUserEcecute {
        SearchUserEcecute::new(Box::new(SearchUserHandle::new(Box::new(UserDB))))
    }

    pub fn get_all_tweets() -> GetAllTweetsExecute {
        GetAllTweetsExecute::new(Box::new(GetAllTweetsHandler::new(Box::new(TweetDB))))
    }
}
