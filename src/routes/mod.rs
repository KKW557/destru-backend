mod structures;

use actix_web::web;
use crate::routes::structures::{get_structure, get_structures};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(get_structure)
            .service(get_structures)
    );
}