mod structures;
mod users;
mod auths;

use actix_web::web;
use crate::routes::auths::{login, logout, register};
use crate::routes::structures::{get_structure, get_structures};
use crate::routes::users::{get_user, get_user_by, get_users};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(register)
            .service(login)
            .service(logout)
            .service(get_structure)
            .service(get_structures)
            .service(get_user)
            .service(get_user_by)
            .service(get_users)
    );
}