use actix_web::error::ErrorBadRequest;
use actix_web::{post, web, Responder, Result};

use crate::api::composition::Composition;
use crate::api::request::PostTweetRequest;
use crate::api::response::PostTweetResponse;

#[utoipa::path(
    post,
    request_body = PostTweetRequest,
    responses(
        (status = 200, description = "Post Tweet", body = PostTweetResponse),
        (status = 400, description = "Bad request")
    ),
)]
#[post("/post")]
pub async fn post_tweet(req: web::Json<PostTweetRequest>) -> Result<impl Responder> {
    let name = req.0.name();
    let content = req.0.content();
    let Ok(tweet) = Composition::post_tweet().run(name, content).await
        else {return Err(ErrorBadRequest("BadRequest"))};
    Ok(web::Json(PostTweetResponse::from(tweet)))
}
