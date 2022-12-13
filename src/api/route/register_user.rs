use actix_web::error::ErrorInternalServerError;
use actix_web::{post, web, Responder, Result};

use crate::api::composition::Composition;
use crate::api::execute::RegisterUserExecute;
use crate::api::request::RegisterUserRequest;

#[utoipa::path(
    post,
    request_body = RegisterUserRequest,
    responses(
        (status = 200, description = "Register User", body = RegisterUserRequest),
        (status = 500, description = "Internal error")
    ),
)]
#[post("/register")]
pub async fn register_user(
    data: web::Data<Composition>,
    req: web::Json<RegisterUserRequest>,
) -> Result<impl Responder> {
    let Ok(response) = RegisterUserExecute::new(data.register_user()).run(req.0).await
        else {return Err(ErrorInternalServerError("InternalError"))};
    Ok(web::Json(response))
}
