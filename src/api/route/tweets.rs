use actix_web::error::ErrorNotFound;
use actix_web::{get, web, Responder, Result};

use crate::api::composition::Composition;
use crate::api::request::NamePath;
use crate::api::response::GetAllTweetResponse;
#[utoipa::path(
    get,
    context_path = "/tweets",
    params(("name", description = "User Name")),
    responses(
        (status = 200, description = "Get All Tweets", body = GetAllTweetResponse),
        (status = 404, description = "Not found")
    ),
)]
#[get("/{name}")]
pub async fn get_all_tweets(req: web::Path<NamePath>) -> Result<impl Responder> {
    let name = req.name.to_string();
    let Ok(tweets) = Composition::get_all_tweets().run(name).await
        else {return Err(ErrorNotFound("NotFound"))};
    Ok(web::Json(GetAllTweetResponse::from(tweets)))
}
