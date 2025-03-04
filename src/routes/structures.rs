use crate::models::files::File;
use crate::models::pagination::PaginationParams;
use crate::models::responses::{NotFoundResponse, StructureResponse, StructuresResponse};
use crate::models::structures::{DbStructure, Structure, StructurePreview};
use crate::models::users::User;
use actix_web::web::{Data, Path, Query};
use actix_web::{get, HttpResponse, Responder};
use sqlx::PgPool;
use destru::decode;
use crate::models::ids::STRUCTURE_FLAG;

#[get("/structure/{id}")]
pub async fn get_structure(id: Path<String>, postgre: Data<PgPool>) -> impl Responder {
    match decode(STRUCTURE_FLAG, id.as_str()) {
        Ok(id) => {
            let mut tx = postgre.begin().await.unwrap();

            let db_structure = sqlx::query_as!(
                DbStructure,
                r#"SELECT id, name, summary, description, created FROM structures WHERE id = $1"#,
                id,
            )
            .fetch_one(&mut *tx)
            .await
            .unwrap();

            let files = sqlx::query_as!(
                File,
                r#"SELECT url, created FROM structure_files WHERE structure = $1"#,
                id,
            )
            .fetch_all(&mut *tx)
            .await
            .unwrap();

            let images = sqlx::query_as!(
                File,
                r#"SELECT url, created FROM structure_images WHERE structure = $1"#,
                id,
            )
            .fetch_all(&mut *tx)
            .await
            .unwrap();

            let creators = sqlx::query_as!(
                User,
                r#"SELECT u.id, u.name, u.avatar FROM structure_creators c JOIN users u ON c.id = u.id WHERE c.structure = $1"#,
                id,
            )
                .fetch_all(&mut *tx)
                .await
                .unwrap();

            tx.commit().await.expect("failed to commit transaction");

            HttpResponse::Ok().json(StructureResponse::new(
                    Structure {
                        id: db_structure.id,
                        name: db_structure.name,
                        summary: db_structure.summary,
                        description: db_structure.description,
                        created: db_structure.created,
                        files,
                        images,
                        creators,
            }))
        }
        Err(_) => HttpResponse::NotFound().json(NotFoundResponse::new()),
    }
}

#[get("/structures")]
pub async fn get_structures(params: Query<PaginationParams>, postgre: Data<PgPool>) -> impl Responder {
    let page = params.page.unwrap_or(1);
    let size = params.size.unwrap_or(24);

    let mut tx = postgre.begin().await.unwrap();

    let structures = sqlx::query_as!(
        StructurePreview,
        r#"
        SELECT
            s.id,
            s.name,
            si.url as image,
            u.name as creator
        FROM structures s
        LEFT JOIN LATERAL (
            SELECT url
            FROM structure_images
            WHERE structure = s.id
            ORDER BY id
            LIMIT 1
        ) si ON true
        LEFT JOIN LATERAL (
            SELECT creator
            FROM structure_creators
            WHERE structure = s.id
            ORDER BY id
            LIMIT 1
        ) sc ON true
        LEFT JOIN users u ON u.id = sc.creator
        LIMIT $1
        OFFSET $2
        "#,
        size,
        (page - 1) * size,
    )
        .fetch_all(&mut *tx)
        .await
        .unwrap();

    tx.commit().await.expect("failed to commit transaction");

    HttpResponse::Ok().json(StructuresResponse::new(structures))
}
