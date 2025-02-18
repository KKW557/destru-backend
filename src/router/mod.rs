use crate::handler::{structure, structures};
use actix_web::web;
use actix_web::web::get;

pub fn v1(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/v1")
            .route("/structures", get().to(structures::get))
            .route("/structure/{id}", get().to(structure::get))
    );
}