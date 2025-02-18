use crate::model::file::File;
use crate::model::structure::Structure;
use crate::model::user::UserPreview;
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{query_as, FromRow, MySqlPool};

#[derive(Serialize)]
struct StructureResponse {
    structure: Structure,
    timestamp: i64,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct DbStructure {
    pub id: String,
    pub name: String,
    pub summary: String,
    pub description: String,
    pub created: i64,
}

pub async fn get(id: web::Path<String>, mysql: web::Data<MySqlPool>) -> impl Responder {
    let mut tx = mysql.begin().await.unwrap();

    let id = id.to_string();

    let db_structure = query_as::<_, DbStructure>(
        r#"SELECT id, name, summary, description, created FROM structures WHERE id = ?"#
    )
        .bind(id.clone())
        .fetch_one(&mut *tx)
        .await
        .unwrap();

    let files = query_as::<_, File>(
        r#"SELECT url, created FROM structure_files WHERE structure = ?"#
    )
        .bind(id.clone())
        .fetch_all(&mut *tx)
        .await
        .unwrap();

    let images = query_as::<_, File>(
        r#"SELECT url, created FROM structure_images WHERE structure = ?"#
    )
        .bind(id.clone())
        .fetch_all(&mut *tx)
        .await
        .unwrap();

    let creators = query_as::<_, UserPreview>(
        r#"SELECT u.id, u.name, u.avatar FROM structure_creators c JOIN users u ON c.id = u.id WHERE c.structure = ?"#
    )
        .bind(id.clone())
        .fetch_all(&mut *tx)
        .await
        .unwrap();

    HttpResponse::Ok().json(StructureResponse {
        structure: Structure {
            id: db_structure.id,
            name: db_structure.name,
            summary: db_structure.summary,
            description: db_structure.description,
            created: db_structure.created,
            files,
            images,
            creators,
        },
        timestamp: Utc::now().timestamp(),
    })
}
