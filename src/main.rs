mod model;
mod handler;
mod config;
mod router;
mod database;

use crate::database::mysql::mysql;
use crate::router::v1;
use actix_web::{App, HttpServer};
use actix_web::web::Data;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::new();

    let mysql = mysql(config.database.url).await.expect("Failed to connect to MySQL");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(mysql.clone()))
            .configure(v1)
    })
        .bind((config.server.addr, config.server.port))?
        .run()
        .await
}