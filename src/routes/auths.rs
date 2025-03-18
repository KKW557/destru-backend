use actix_web::{post, HttpRequest, HttpResponse, Responder};
use actix_web::web::{Data, Json};
use sqlx::PgPool;
use chrono::{Duration, Utc};
use actix_web::cookie::time::OffsetDateTime;
use actix_web::cookie::{Cookie, SameSite};
use lazy_static::lazy_static;
use regex::Regex;
use destru::{generate_jwt, hash_password, verify_password};
use crate::models::ids::UserID;
use crate::models::responses::{LoginResponse, RegisterErrorResponse, UserError};
use crate::models::users::{UserLogin, UserRegister};

lazy_static! {
    static ref NAME_REGEX: Regex = Regex::new(r"^[0-9a-zA-Z_-]{3,100}$").unwrap();
    static ref PASSWORD_REGEX: Regex = Regex::new(r"^[0-9a-fA-F]{64}$").unwrap();
}

#[post("/auths/register")]
pub async fn register(register: Json<UserRegister>, postgre: Data<PgPool>) -> impl Responder {
    let name = register.name.clone().unwrap_or_default();
    if !NAME_REGEX.is_match(&name) {
        return HttpResponse::BadRequest().json(RegisterErrorResponse {
            reason: UserError::InvalidName
        });
    }

    let password = register.password.clone().unwrap_or_default();
    if !PASSWORD_REGEX.is_match(&password) {
        return HttpResponse::BadRequest().json(RegisterErrorResponse {
            reason: UserError::InvalidPassword
        });
    }

    let mut tx = postgre.begin().await.unwrap();

    let name_exists = sqlx::query_scalar!(
        r"SELECT EXISTS(SELECT 1 FROM users WHERE name = $1)",
        &name
    )
        .fetch_one(&mut *tx)
        .await
        .unwrap()
        .unwrap();

    if name_exists {
        tx.commit().await.expect("failed to commit transaction");

        return HttpResponse::BadRequest().json(RegisterErrorResponse {
            reason: UserError::NameExists
        });
    }

    let password = hash_password(password.as_str()).unwrap();

    sqlx::query!(
        r"INSERT INTO users (name, password) VALUES ($1, $2)",
        name,
        password
    )
        .execute(&mut *tx)
        .await
        .expect("failed to insert user");

    tx.commit().await.expect("failed to commit transaction");

    HttpResponse::Ok().finish()
}

#[post("/auths/login")]
pub async fn login(login: Json<UserLogin>, postgre: Data<PgPool>) -> impl Responder {
    let name = login.name.clone().unwrap_or_default();
    if name.is_empty() {
        return HttpResponse::BadRequest().json(RegisterErrorResponse {
            reason: UserError::InvalidName,
        });
    }

    let password = login.password.clone().unwrap_or_default();
    if password.is_empty() {
        return HttpResponse::BadRequest().json(RegisterErrorResponse {
            reason: UserError::InvalidPassword,
        });
    }

    let mut tx = postgre.begin().await.unwrap();

    let row = sqlx::query!(
        r"SELECT id, password FROM users WHERE name = $1",
        name
    )
        .fetch_optional(&mut *tx)
        .await;

    match row {
        Ok(Some(record)) => {
            let id = record.id;
            let hash = record.password.unwrap_or_default();

            if verify_password(&password, &hash) {
                let expired = if login.remember.unwrap_or(false) {
                    Utc::now() + Duration::days(30)
                } else {
                    Utc::now() + Duration::hours(24)
                };

                let token = generate_jwt(id, expired);

                sqlx::query!(
                    r#"DELETE FROM user_tokens WHERE "user" = $1 AND expired < NOW()"#,
                    id
                )
                    .execute(&mut *tx)
                    .await
                    .expect("failed to delete expired tokens");

                sqlx::query!(
                    r#"DELETE FROM user_tokens WHERE "user" = $1 AND id NOT IN (SELECT id FROM user_tokens WHERE "user" = $1 ORDER BY id DESC LIMIT 5)"#,
                    id
                )
                    .execute(&mut *tx)
                    .await
                    .expect("failed to delete excess tokens");

                sqlx::query!(
                    r#"INSERT INTO user_tokens ("user", token, expired) VALUES ($1, $2, $3)"#,
                    id,
                    token,
                    expired.naive_utc()
                )
                    .execute(&mut *tx)
                    .await
                    .expect("failed to insert new token");

                tx.commit().await.expect("failed to commit transaction");

                let when = OffsetDateTime::from_unix_timestamp(expired.timestamp())
                    .expect("failed to get expired offset");

                let cookie: Cookie = {
                    #[cfg(debug_assertions)]
                    {
                        Cookie::build("Token", token)
                            .http_only(true)
                            .secure(true)
                            .same_site(SameSite::None)
                            .path("/")
                            .expires(when)
                            .finish()
                    }
                    #[cfg(not(debug_assertions))]
                    {
                        Cookie::build("Token", token)
                            .http_only(true)
                            .secure(true)
                            .same_site(actix_web::cookie::SameSite::Lax)
                            .path("/")
                            .expires(when)
                            .domain("destru.org")
                            .finish()
                    }
                };

                HttpResponse::Ok()
                    .cookie(cookie)
                    .json(LoginResponse {
                        id: UserID::from(id),
                    })
            } else {
                tx.commit().await.expect("failed to commit transaction");

                HttpResponse::Unauthorized().finish()
            }
        }
        Ok(None) => {
            tx.commit().await.expect("failed to commit transaction");

            HttpResponse::NotFound().finish()
        }
        Err(_) => {
            tx.commit().await.expect("failed to commit transaction");

            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/auths/logout")]
pub async fn logout(req: HttpRequest, postgre: Data<PgPool>) -> impl Responder {
    let cookie = req.cookie("Token")
        .ok_or_else(|| HttpResponse::Unauthorized().finish())
        .unwrap();
    let token = cookie.value();

    let mut tx = postgre.begin().await.unwrap();

    sqlx::query!(
        r"DELETE FROM user_tokens WHERE token = $1",
        token
    )
        .execute(&mut *tx)
        .await
        .expect("failed to delete expired tokens");

    tx.commit().await.expect("failed to commit transaction");

    let cookie: Cookie = {
        #[cfg(debug_assertions)]
        {
            Cookie::build("Token", "")
                .http_only(true)
                .secure(true)
                .same_site(SameSite::None)
                .path("/")
                .max_age(time::Duration::ZERO)
                .finish()
        }
        #[cfg(not(debug_assertions))]
        {
            Cookie::build("Token", "")
                .http_only(true)
                .secure(true)
                .same_site(SameSite::Lax)
                .path("/")
                .domain("destru.org")
                .max_age(time::Duration::ZERO)
                .finish()
        }
    };

    HttpResponse::Ok().cookie(cookie).finish()
}