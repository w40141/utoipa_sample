use actix_web::{get, HttpResponse, Responder};

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
