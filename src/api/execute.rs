use anyhow::Result;

use crate::usecase::{
    GetAllTweetsUsecase, PostTweetUsecase, RegisterUserUsecase, SearchUserUsecase,
};

use super::request::{PostTweetRequest, RegisterUserRequest};
use super::response::{
    GetAllTweetResponse, PostTweetResponse, RegisterUserResponse, SearchedUserResponse,
};

pub struct RegisterUserExecute;

impl RegisterUserExecute {
    pub async fn run(
        usecase: Box<dyn RegisterUserUsecase>,
        req: RegisterUserRequest,
    ) -> Result<RegisterUserResponse> {
        let u = usecase.handle(req.name(), req.email());
        Ok(RegisterUserResponse::from(u))
    }
}

pub struct PostTweetExecute;

impl PostTweetExecute {
    pub async fn run(
        usecase: Box<dyn PostTweetUsecase>,
        req: PostTweetRequest,
    ) -> Result<PostTweetResponse> {
        let u = usecase.handle(req.name(), req.content())?;
        Ok(PostTweetResponse::from(u))
    }
}

pub struct SearchUserExecute;

impl SearchUserExecute {
    pub async fn run(
        usecase: Box<dyn SearchUserUsecase>,
        req: &str,
    ) -> Result<SearchedUserResponse> {
        let u = usecase.handle(req)?;
        Ok(SearchedUserResponse::from(u))
    }
}

pub struct GetAllTweetsExecute;

impl GetAllTweetsExecute {
    pub async fn run(
        usecase: Box<dyn GetAllTweetsUsecase>,
        req: &str,
    ) -> Result<GetAllTweetResponse> {
        let u = usecase.handle(req)?;
        Ok(GetAllTweetResponse::from(u))
    }
}
