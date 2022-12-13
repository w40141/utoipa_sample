use actix_web::error::ErrorNotFound;
use actix_web::{get, web, Responder, Result};

use crate::api::composition::Composition;
use crate::api::execute::GetAllTweetsExecute;
use crate::api::request::Name;
use crate::api::response::GetAllTweetResponse;

#[utoipa::path(
    get,
    context_path = "/tweets",
    params(Name),
    responses(
        (status = 200, description = "Get All Tweets", body = GetAllTweetResponse),
        (status = 404, description = "Not found")
    ),
)]
#[get("/{name}")]
pub async fn get_all_tweets(
    data: web::Data<Composition>,
    req: web::Path<Name>,
) -> Result<impl Responder> {
    let Ok(tweets) = GetAllTweetsExecute::new(data.get_all_tweets()).run(req.name()).await
        else {return Err(ErrorNotFound("NotFound"))};
    Ok(web::Json(GetAllTweetResponse::from(tweets)))
}
