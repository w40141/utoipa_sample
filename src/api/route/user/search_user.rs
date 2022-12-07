use actix_web::error::ErrorNotFound;
use actix_web::{get, web, Responder, Result};

use crate::api::composition::Composition;
use crate::api::request::NamePath;
use crate::api::response::SearchedUserResponse;

#[utoipa::path(
    get,
    context_path = "/user",
    params(("name", description = "User Name")),
    responses(
        (status = 200, description = "Search User", body = SearchedUserResponse),
        (status = 404, description = "Not found")
    ),
)]
#[get("/{name}")]
pub async fn search_user(req: web::Path<NamePath>) -> Result<impl Responder> {
    let name = req.name();
    let Ok(user) = Composition::search_user().run(name).await
        else { return Err(ErrorNotFound("NotFound"))};
    Ok(web::Json(SearchedUserResponse::from(user)))
}
