use actix_web::error::{ErrorBadRequest, ErrorNotFound};
use actix_web::{get, post, web, HttpResponse, Responder, Result};

use crate::api::request::{PostTweetRequest, RegisteredUserRequest};
use crate::api::response::{
    GetAllTweetResponse, PostTweetResponse, RegisteredUserResponse, SearchedUserResponse,
};
use crate::infra::tweet::TweetDB;
use crate::infra::user::UserDB;
use crate::usecase::get_all_tweets::GetAllTweetsHandler;
use crate::usecase::post_tweet::PostTweetHandler;
use crate::usecase::register_user::RegisterUserHandler;
use crate::usecase::search_user::SearchUserHandle;
use crate::usecase::{GetAllTweets, PostTweet, RegisterUser, SearchUser};

#[get("/check_health")]
pub async fn check_health() -> impl Responder {
    HttpResponse::Ok().body("Running service")
}

#[post("/register")]
pub async fn register_user(req: web::Json<RegisteredUserRequest>) -> Result<impl Responder> {
    let name = req.name.to_string();
    let email = req.email.to_string();
    let handler = RegisterUserHandler::new(Box::new(UserDB));
    let user = handler.handle(name, email);
    Ok(web::Json(RegisteredUserResponse::from(user)))
}

#[post("/post")]
pub async fn post_tweet(req: web::Json<PostTweetRequest>) -> Result<impl Responder> {
    let user_name = req.name.to_string();
    let content = req.content.to_string();
    let handler = PostTweetHandler::new(Box::new(TweetDB), Box::new(UserDB));
    let Ok(tweet) = handler.handle(user_name, content) else {return Err(ErrorBadRequest("BadRequest"))};
    Ok(web::Json(PostTweetResponse::from(tweet)))
}

#[get("/search_user/{name}")]
pub async fn search_user(name: web::Path<String>) -> Result<impl Responder> {
    let handler = SearchUserHandle::new(Box::new(UserDB));
    let Ok(user) = handler.handle(name.to_string()) else { return Err(ErrorNotFound("NotFound"))};
    Ok(web::Json(SearchedUserResponse::from(user)))
}

#[get("/tweets/{name}")]
pub async fn get_all_tweets(name: web::Path<String>) -> Result<impl Responder> {
    let handler = GetAllTweetsHandler::new(Box::new(TweetDB));
    let Ok(tweets) = handler.handle(name.to_string()) else {return Err(ErrorNotFound("NotFound"))};
    Ok(web::Json(GetAllTweetResponse::from(tweets)))
}
