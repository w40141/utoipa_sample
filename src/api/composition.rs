use crate::infra::user::UserDB;
use crate::usecase::get_all_tweets::GetAllTweetsHandler;
use crate::usecase::post_tweet::PostTweetHandler;
use crate::usecase::register_user::RegisterUserHandler;
use crate::usecase::search_user::SearchUserHandle;
use crate::usecase::{PostTweetUsecase, SearchUserUsecase, GetAllTweetsUsecase};
use crate::{infra::tweet::TweetDB, usecase::RegisterUserUsecase};

/// 依存性注入
pub struct Composition;

impl Composition {
    pub fn register_user(&self) -> Box<impl RegisterUserUsecase> {
        Box::new(RegisterUserHandler::new(Box::new(UserDB)))
    }

    pub fn post_tweet(&self) -> Box<impl PostTweetUsecase> {
        Box::new(PostTweetHandler::new(
            Box::new(TweetDB),
            Box::new(UserDB),
        ))
    }

    pub fn search_user(&self) -> Box<impl SearchUserUsecase> {
        Box::new(SearchUserHandle::new(Box::new(UserDB)))
    }

    pub fn get_all_tweets(&self) -> Box<impl GetAllTweetsUsecase> {
        Box::new(GetAllTweetsHandler::new(Box::new(TweetDB)))
    }
}
