pub mod search_user;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user").service(search_user::search_user));
}
