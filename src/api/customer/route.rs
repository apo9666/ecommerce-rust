use crate::api::customer;
use actix_web::web;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(customer::controller::get_customer);
    cfg.service(customer::controller::create_customer);
}
