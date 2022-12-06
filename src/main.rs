pub mod api;
pub mod domain;
pub mod infra;
pub mod usecase;

use actix_web::{App, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};

use crate::api::request::*;
use crate::api::response::*;
use crate::api::route;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            route::check_health,
            route::register_user,
            route::post_tweet,
            route::search_user,
            route::get_all_tweets
        ),
        components(
            schemas()
        ),
        tags(
            (name = "todo", description = "Todo management endpoints.")
        ),
        modifiers(&SecurityAddon)
    )]
    struct ApiDoc;

    HttpServer::new(|| {
        App::new()
            .service(route::check_health)
            .service(route::register_user)
            .service(route::post_tweet)
            .service(route::get_all_tweets)
            .service(route::search_user)
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").urls(vec!["/api-doc/opanapi.json"]))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
