use anyhow::Result;

use crate::domain::tweet::{Tweet, Tweets};
use crate::domain::user::User;
use crate::usecase::{
    GetAllTweetsUsecase, PostTweetUsecase, RegisterUserUsecase, SearchUserUsecase,
};

pub struct RegisterUserExecute {
    usecase: Box<dyn RegisterUserUsecase>,
}

impl RegisterUserExecute {
    pub fn new(usecase: Box<dyn RegisterUserUsecase>) -> Self {
        Self { usecase }
    }

    pub async fn run(&self, name: &str, email: &str) -> Result<User> {
        Ok(self.usecase.handle(name, email))
    }
}

pub struct PostTweetExecute {
    usecase: Box<dyn PostTweetUsecase>,
}

impl PostTweetExecute {
    pub fn new(usecase: Box<dyn PostTweetUsecase>) -> Self {
        Self { usecase }
    }

    pub async fn run(&self, user_name: &str, content: &str) -> Result<Tweet> {
        self.usecase.handle(user_name, content)
    }
}

pub struct SearchUserEcecute {
    usecase: Box<dyn SearchUserUsecase>,
}

impl SearchUserEcecute {
    pub fn new(usecase: Box<dyn SearchUserUsecase>) -> Self {
        Self { usecase }
    }

    pub async fn run(&self, name: &str) -> Result<User> {
        self.usecase.handle(name)
    }
}

pub struct GetAllTweetsExecute {
    usecase: Box<dyn GetAllTweetsUsecase>,
}

impl GetAllTweetsExecute {
    pub fn new(usecase: Box<dyn GetAllTweetsUsecase>) -> Self {
        Self { usecase }
    }

    pub async fn run(&self, name: &str) -> Result<Tweets> {
        self.usecase.handle(name)
    }
}
