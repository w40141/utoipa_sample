use actix_web::error::{ErrorBadRequest, ErrorInternalServerError, ErrorNotFound};
use actix_web::{get, post, web, HttpResponse, Responder, Result};

use crate::api::composition::Composition;
use crate::api::request::{PostTweetRequest, RegisterUserRequest};
use crate::api::response::{
    GetAllTweetResponse, PostTweetResponse, RegisteredUserResponse, SearchedUserResponse,
};

#[get("/check_health")]
pub async fn check_health() -> impl Responder {
    HttpResponse::Ok().body("Running service")
}

#[post("/register")]
pub async fn register_user(req: web::Json<RegisterUserRequest>) -> Result<impl Responder> {
    let name = req.name.to_string();
    let email = req.email.to_string();
    let Ok(user) = Composition::register_user().run(name, email).await else {return Err(ErrorInternalServerError("InternalError"))};
    Ok(web::Json(RegisteredUserResponse::from(user)))
}

#[post("/post")]
pub async fn post_tweet(req: web::Json<PostTweetRequest>) -> Result<impl Responder> {
    let PostTweetRequest { name, content } = req.0;
    let Ok(tweet) = Composition::post_tweet().run(name, content).await else {return Err(ErrorBadRequest("BadRequest"))};
    Ok(web::Json(PostTweetResponse::from(tweet)))
}

#[get("/user/{name}")]
pub async fn search_user(name: web::Path<String>) -> Result<impl Responder> {
    let Ok(user) = Composition::search_user().run(name.to_string()).await else { return Err(ErrorNotFound("NotFound"))};
    Ok(web::Json(SearchedUserResponse::from(user)))
}

#[get("/tweets/{name}")]
pub async fn get_all_tweets(name: web::Path<String>) -> Result<impl Responder> {
    let Ok(tweets) = Composition::get_all_tweets().run(name.to_string()).await else {return Err(ErrorNotFound("NotFound"))};
    Ok(web::Json(GetAllTweetResponse::from(tweets)))
}
