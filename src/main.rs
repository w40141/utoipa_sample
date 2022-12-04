pub mod api;
pub mod usecase;
pub mod domain;

use actix_web::{App, HttpServer};

use crate::api::route;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(route::check_health)
            .service(route::register_user)
            .service(route::post_tweet)
            .service(route::get_all_tweets)
            .service(route::search_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
