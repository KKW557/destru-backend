use actix_web::{get, HttpResponse, Responder};
use actix_web::web::{Data, Path};
use sqlx::PgPool;
use destru::decode_sqids;
use crate::models::ids::USER_FLAG;
use crate::models::responses::UserResponse;
use crate::models::users::User;

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
