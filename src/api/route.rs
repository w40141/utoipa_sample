use actix_web::{get, post, web, HttpResponse, Responder, Result};

use crate::api::request::{
    GetAllTweetRequest, PostTweetRequest, RegisteredUserRequest, SearchedUserRequest,
};
use crate::api::response::{
    GetAllTweetResponse, PostTweetResponse, RegisteredUserResponse, SearchedUserResponse,
};
use crate::usecase::search_user;

#[get("/check_health")]
pub async fn check_health() -> impl Responder {
    HttpResponse::Ok().body("Running service")
}

#[post("/register")]
async fn register_user(req: web::Json<RegisteredUserRequest>) -> Result<impl Responder> {
    let name = req.name.to_string();
    let email = req.email.to_string();
    let obj = ::new(name, email);
    Ok(web::Json(obj))
}

#[post("/post")]
async fn post_tweet(req: web::Json<PostTweetRequest>) -> Result<impl Responder> {
    let user_id = req.user_id;
    let content = req.content;
    let user = search_user(user_id);
    let obj = PostTweetResponse::new(user, content);
    Ok(web::Json(obj))
}

#[get("/search_user/{id}")]
async fn search_user(id: web::Path<u64>) -> Result<impl Responder> {
    let user = search_user(id);
    let obj = SearchedUserResponse::from(user);
    Ok(web::Json(obj))
}

#[get("/tweet/{id}")]
async fn get_all_tweets(id: web::Path<u64>) -> Result<impl Responder> {
    let tweets = todo!();
    let obj = GetAllTweetResponse::from(user, tweets);
    Ok(web::Json(obj))
}
