use actix_web::error::ErrorBadRequest;
use actix_web::{post, web, Responder, Result};

use crate::api::composition::Composition;
use crate::api::execute::PostTweetExecute;
use crate::api::request::PostTweetRequest;

#[utoipa::path(
    post,
    request_body = PostTweetRequest,
    responses(
        (status = 200, description = "Post Tweet", body = PostTweetResponse),
        (status = 400, description = "Bad request")
    ),
)]
#[post("/post")]
pub async fn post_tweet(
    data: web::Data<Composition>,
    req: web::Json<PostTweetRequest>,
) -> Result<impl Responder> {
    let Ok(response) = PostTweetExecute::new(data.post_tweet()).run(req.0).await
        else {return Err(ErrorBadRequest("BadRequest"))};
    Ok(web::Json(response))
}
