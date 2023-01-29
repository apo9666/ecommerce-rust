use crate::api::user;
use actix_web::web;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(user::controller::create_user);
    cfg.service(user::controller::get_user);
}
