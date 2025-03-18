mod structures;
mod users;

use actix_web::web;
use crate::routes::structures::{get_structure, get_structures};
use crate::routes::users::{get_user, get_user_by, get_users, login, register};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(get_structure)
            .service(get_structures)
            .service(register)
            .service(login)
            .service(get_user)
            .service(get_user_by)
            .service(get_users)
    );
}