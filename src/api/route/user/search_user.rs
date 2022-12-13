use actix_web::error::ErrorNotFound;
use actix_web::{get, web, Responder, Result};

use crate::api::composition::Composition;
use crate::api::execute::SearchUserEcecute;
use crate::api::request::Name;

#[utoipa::path(
    get,
    context_path = "/user",
    params(Name),
    responses(
        (status = 200, description = "Search User", body = SearchedUserResponse),
        (status = 404, description = "Not found")
    ),
)]
#[get("/{name}")]
pub async fn search_user(
    data: web::Data<Composition>,
    req: web::Path<Name>,
) -> Result<impl Responder> {
    let Ok(user) = SearchUserEcecute::run(data.search_user(), req.name()).await
        else { return Err(ErrorNotFound("NotFound"))};
    Ok(web::Json(user))
}
