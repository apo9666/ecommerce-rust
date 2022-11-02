use crate::api::health_check::health_check;
use actix_web::web;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check);
}
