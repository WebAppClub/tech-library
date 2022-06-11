use actix_web::web;

use crate::controllers::health_check;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health_check", web::get().to(health_check));
}
