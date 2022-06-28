use actix_web::web;

use crate::controllers::health_check;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").route("/health_check", web::get().to(health_check)));
}
