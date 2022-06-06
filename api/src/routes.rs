use actix_web::{web};
use crate::controllers::index;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index::index));
}
