pub mod api;
pub mod domain;
pub mod infra;
pub mod usecase;

use actix_web::{App, HttpServer};

use crate::api::openapi::config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(config))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
