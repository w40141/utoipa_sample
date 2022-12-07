pub mod get_all_tweets;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/tweets").service(get_all_tweets::get_all_tweets));
}
