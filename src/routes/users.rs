use crate::models::users::{User, UserLogin, UserRegister};
use actix_web::web::{Data, Json, Path};
use actix_web::{get, post, HttpResponse, Responder};
use actix_web::cookie::Cookie;
use actix_web::cookie::time::OffsetDateTime;
use chrono::{Duration, Utc};
use lazy_static::lazy_static;
use sqlx::{PgPool};
use regex::Regex;
use destru::{decode_sqids, generate_jwt, hash_password, verify_password};
use crate::models::ids::{UserID, USER_FLAG};
use crate::models::responses::{LoginResponse, RegisterErrorResponse, UserError, UserResponse};

lazy_static! {
    static ref NAME_REGEX: Regex = Regex::new(r"^[0-9a-zA-Z_-]{3,100}$").unwrap();
    static ref PASSWORD_REGEX: Regex = Regex::new(r"^[0-9a-fA-F]{64}$").unwrap();
}

#[post("/register")]
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

#[post("/login")]
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

                let cookie = {
                    #[cfg(debug_assertions)]
                    {
                        Cookie::build("Token", token)
                            .http_only(true)
                            .secure(true)
                            .same_site(actix_web::cookie::SameSite::None)
                            .path("/")
                            .expires(when)
                            .finish()
                    }
                    #[cfg(not(debug_assertions))]
                    {
                        Cookie::build("token", token)
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

#[get("/users")]
pub async fn get_users() -> impl Responder {
    HttpResponse::Unauthorized().finish()
}

async fn get_user_by_id_response(id: &str, postgre: Data<PgPool>) -> HttpResponse {
    match decode_sqids(USER_FLAG, id) {
        Ok(id) => {
            let mut tx = postgre.begin().await.unwrap();

            let user = sqlx::query_as!(
        User,
        r"SELECT id, name, avatar, slug, bio FROM users WHERE id = $1",
        id
    )
                .fetch_one(&mut *tx)
                .await;

            tx.commit().await.expect("failed to commit transaction");

            match user {
                Ok(user) => HttpResponse::Ok().json(UserResponse { user }),
                Err(e) => {
                    println!("{:?}", e);
                    HttpResponse::NotFound().finish()
                },
            }
        }
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

async fn get_user_by_name_response(name: &str, postgre: Data<PgPool>) -> HttpResponse {
    let mut tx = postgre.begin().await.unwrap();

    let user = sqlx::query_as!(
        User,
        r"SELECT id, name, avatar, slug, bio FROM users WHERE name = $1",
        name
    )
        .fetch_one(&mut *tx)
        .await;

    tx.commit().await.expect("failed to commit transaction");

    match user {
        Ok(user) => HttpResponse::Ok().json(UserResponse { user }),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/users/{id}")]
pub async fn get_user(id: Path<String>, postgre: Data<PgPool>) -> impl Responder {
    get_user_by_id_response(id.as_str(), postgre).await
}

#[get("/users/by/{key}/{value}")]
pub async fn get_user_by(path: Path<(String, String)>, postgre: Data<PgPool>) -> impl Responder {
    let (key, value) = path.into_inner();
    match key.as_str() {
        "id" => get_user_by_id_response(value.as_str(), postgre).await,
        "name" => get_user_by_name_response(value.as_str(), postgre).await,
        _ => HttpResponse::NotFound().finish(),
    }
}
