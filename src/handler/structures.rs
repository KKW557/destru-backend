use crate::model::pagination::{Pagination, PaginationParams};
use crate::model::structure::StructurePreview;
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use serde::Serialize;

#[derive(Serialize)]
struct StructuresResponse {
    structures: Vec<StructurePreview>,
    pagination: Pagination,
    timestamp: i64,
}

pub async fn get(params: web::Query<PaginationParams>) -> impl Responder {
    let page = params.get_page();
    let size = params.get_size();

    HttpResponse::Ok().json(StructuresResponse {
        structures: vec!(StructurePreview {
            id: "3213232".to_string(),
            name: "好看的樱花树".to_string(),
            image: "https://baidu.com".to_string(),
            creator: "zm127".to_string()
        }),
        pagination: Pagination {
            page,
            size,
            total: page + 1,
        },
        timestamp: Utc::now().timestamp(),
    })
}