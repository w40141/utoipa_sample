pub mod tweets;
pub mod user;

use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use actix_web::{get, post, web, HttpResponse, Responder, Result};

use crate::api::composition::Composition;
use crate::api::request::{PostTweetRequest, RegisterUserRequest};
use crate::api::response::{PostTweetResponse, RegisterUserResponse};

#[utoipa::path(
    get,
    path = "/check_health",
    responses(
        (status = 200, description = "Check health", body = String)
))]
#[get("/check_health")]
pub async fn check_health() -> impl Responder {
    HttpResponse::Ok().body("Running service")
}

#[utoipa::path(
    post,
    // path = "/post",
    request_body = PostTweetRequest,
    responses(
        (status = 200, description = "Post Tweet", body = PostTweetResponse),
        (status = 400, description = "Bad request")
    ),
)]
#[post("/post")]
pub async fn post_tweet(req: web::Json<PostTweetRequest>) -> Result<impl Responder> {
    let PostTweetRequest { name, content } = req.0;
    let Ok(tweet) = Composition::post_tweet().run(name, content).await else {return Err(ErrorBadRequest("BadRequest"))};
    Ok(web::Json(PostTweetResponse::from(tweet)))
}

#[utoipa::path(
    post,
    request_body = RegisterUserRequest,
    responses(
        (status = 200, description = "Register User", body = RegisterUserRequest),
        (status = 500, description = "Internal error")
    ),
)]
#[post("/register")]
pub async fn register_user(req: web::Json<RegisterUserRequest>) -> Result<impl Responder> {
    let name = req.name.to_string();
    let email = req.email.to_string();
    let Ok(user) = Composition::register_user().run(name, email).await else {return Err(ErrorInternalServerError("InternalError"))};
    Ok(web::Json(RegisterUserResponse::from(user)))
}
