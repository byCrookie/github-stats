use std::{
    fs,
    io::{self, Error, Read, Write},
    path::Path,
    time::{Duration, SystemTime},
};

use actix_web::{App, Either, get, http::header::{self, CacheControl, CacheDirective}, HttpResponse, HttpServer, middleware::Logger, Responder, web, web::Data};
use actix_web::http::{Method, StatusCode};
use config::{ConfigError, Environment};
use dotenv::dotenv;
use env_logger::Target;
use log::{debug, error, info, LevelFilter};
use mime;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Config {
    address: String,
    port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg_builder = config::Config::builder();
        cfg_builder = cfg_builder.set_default("address", "0.0.0.0")?;
        cfg_builder = cfg_builder.set_default("port", 8080)?;

        cfg_builder = cfg_builder.add_source(
            Environment::default()
                .prefix("STATS")
                .prefix_separator("_")
                .separator("__"),
        );

        let config = cfg_builder.build()?;
        config.try_deserialize()
    }
}

#[get("/health")]
async fn health_endpoint() -> impl Responder {
    HttpResponse::Ok().body("healthy")
}

#[actix_web::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .parse_default_env()
        .target(Target::Stdout)
        .init();

    let config = match Config::from_env() {
        Ok(config) => config,
        Err(err) => {
            error!("Failed to read config: {:?}", err);
            return Err(Error::new(io::ErrorKind::Other, err.to_string()));
        }
    };

    let address: String = config.address.clone();
    let port: u16 = config.port.clone();

    info!("Running on {address}:{port}");

    match HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(config.clone()))
            .service(health_endpoint)
    })
        .bind((address, port))?
        .workers(2)
        .run()
        .await {
        Ok(_) => {
            info!("Server stopped");
        }
        Err(err) => {
            error!("Running server failed: {:?}", err);
            return Err(Error::new(io::ErrorKind::Other, err.to_string()));
        }
    };

    Ok(())
}
