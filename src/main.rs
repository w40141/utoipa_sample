pub mod api;
pub mod domain;
pub mod infra;
pub mod usecase;

use actix_web::{App, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::openapi::{config, ApiDoc};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().configure(config).service(
            SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/opanapi.json", ApiDoc::openapi()),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
