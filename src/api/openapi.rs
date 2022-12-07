use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::request::*;
use crate::api::response::*;
use crate::api::route;

#[derive(OpenApi)]
#[openapi(
    paths(
        route::check_health,
        route::register_user,
        route::post_tweet,
        route::user::search_user,
        route::tweets::get_all_tweets
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
    cfg.service(route::check_health)
        .service(route::register_user)
        .service(route::post_tweet)
        .service(web::scope("/user").service(route::user::search_user))
        .service(web::scope("/tweets").service(route::tweets::get_all_tweets))
        .service(
            SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/opanapi.json", ApiDoc::openapi()),
        );
}
