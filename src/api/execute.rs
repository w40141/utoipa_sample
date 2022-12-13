use anyhow::Result;

use crate::usecase::{
    GetAllTweetsUsecase, PostTweetUsecase, RegisterUserUsecase, SearchUserUsecase,
};

use super::request::{PostTweetRequest, RegisterUserRequest};
use super::response::{
    GetAllTweetResponse, PostTweetResponse, RegisterUserResponse, SearchedUserResponse,
};

pub struct RegisterUserExecute {
    usecase: Box<dyn RegisterUserUsecase>,
}

impl RegisterUserExecute {
    pub fn new(usecase: Box<dyn RegisterUserUsecase>) -> Self {
        Self { usecase }
    }

    pub async fn run(&self, req: RegisterUserRequest) -> Result<RegisterUserResponse> {
        let u = self.usecase.handle(req.name(), req.email());
        Ok(RegisterUserResponse::from(u))
    }
}

pub struct PostTweetExecute {
    usecase: Box<dyn PostTweetUsecase>,
}

impl PostTweetExecute {
    pub fn new(usecase: Box<dyn PostTweetUsecase>) -> Self {
        Self { usecase }
    }

    pub async fn run(&self, req: PostTweetRequest) -> Result<PostTweetResponse> {
        let u = self.usecase.handle(req.name(), req.content())?;
        Ok(PostTweetResponse::from(u))
    }
}

pub struct SearchUserEcecute {
    usecase: Box<dyn SearchUserUsecase>,
}

impl SearchUserEcecute {
    pub fn new(usecase: Box<dyn SearchUserUsecase>) -> Self {
        Self { usecase }
    }

    pub async fn run(&self, req: &str) -> Result<SearchedUserResponse> {
        let u = self.usecase.handle(req)?;
        Ok(SearchedUserResponse::from(u))
    }
}

pub struct GetAllTweetsExecute {
    usecase: Box<dyn GetAllTweetsUsecase>,
}

impl GetAllTweetsExecute {
    pub fn new(usecase: Box<dyn GetAllTweetsUsecase>) -> Self {
        Self { usecase }
    }

    pub async fn run(&self, req: &str) -> Result<GetAllTweetResponse> {
        let u = self.usecase.handle(req)?;
        Ok(GetAllTweetResponse::from(u))
    }
}
