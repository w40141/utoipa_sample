use actix_web::error::ErrorInternalServerError;
use actix_web::{post, web, Responder, Result};

use crate::api::composition::Composition;
use crate::api::request::RegisterUserRequest;
use crate::api::response::RegisterUserResponse;

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
    let name = req.0.name();
    let email = req.0.email();
    let Ok(user) = Composition::register_user().run(name, email).await
        else {return Err(ErrorInternalServerError("InternalError"))};
    Ok(web::Json(RegisterUserResponse::from(user)))
}
