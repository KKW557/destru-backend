mod routes;
mod database;
mod models;

use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use env_logger::Env;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .init();

    let postgre = database::postgre::connect()
        .await
        .expect("Failed to connect to database");

    HttpServer::new(move || {
        let cors = {
            #[cfg(debug_assertions)]
            {
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .supports_credentials()
                    .max_age(3600)
            }
            #[cfg(not(debug_assertions))]
            {
                Cors::default()
                    .allowed_origin("https://destru.org")
                    .allowed_origin("https://api.destru.org")
                    .allow_any_method()
                    .allow_any_header()
                    .supports_credentials()
                    .max_age(3600)
            }
        };

        App::new()
            .wrap(cors)
            .configure(routes::config)
            .app_data(Data::new(postgre.clone()))
    })
        .bind(dotenvy::var("SERVER_ADDR").expect("`SERVER_ADDR` not in .env"))?
        .run()
        .await
}