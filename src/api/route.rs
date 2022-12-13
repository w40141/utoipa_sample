mod check_health;
mod post_tweet;
mod register_user;
mod tweets;
mod user;

use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::request::*;
use crate::api::response::*;
use crate::api::route;

#[derive(OpenApi)]
#[openapi(
    paths(
        route::check_health::check_health,
        route::register_user::register_user,
        route::post_tweet::post_tweet,
        route::user::search_user::search_user,
        route::tweets::get_all_tweets::get_all_tweets,
    ),
    components(schemas(
        RegisterUserRequest,
        PostTweetRequest,
        RegisterUserResponse,
        PostTweetResponse,
        SearchedUserResponse,
        GetAllTweetResponse
    ))
)]
struct ApiDoc;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(check_health::check_health)
        .service(register_user::register_user)
        .service(post_tweet::post_tweet)
        .configure(user::config)
        .configure(tweets::config)
        .service(
            SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/opanapi.json", ApiDoc::openapi()),
        );
}
