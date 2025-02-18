use ::config::{Config, File};
use serde::Deserialize;

pub fn new() -> AppConfig {
    Config::builder()
        .add_source(File::with_name("config.toml"))
        .build()
        .expect("Unable to build config")
        .try_deserialize()
        .expect("Unable to deserialize config")
}

#[derive(Deserialize)]
pub struct AppConfig {
    pub server: Server,
    pub database: Database,
}

#[derive(Deserialize)]
pub struct Server {
    pub addr: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct Database {
    pub url: String,
}