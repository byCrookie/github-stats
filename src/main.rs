use actix_web::{get, http::header, web::Data, App, HttpResponse, HttpServer, Responder};
use clap::Parser;
use config::{ConfigError, Environment, File};
use dotenv::dotenv;
use mime;
use serde::{Deserialize, Serialize};
use std::{
    io::{self, Error},
    time::SystemTime,
};
use thiserror::Error;

mod stats;
mod toplangs;

const ONE_DAY: u64 = 86400;

#[derive(Error, Debug)]
pub enum StatsError {
    #[error("invalid config")]
    InvalidConfig(#[from] io::Error),
    #[error("unknown error")]
    Unknown,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Config {
    cache_seconds: u64,
    github_token: String,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "")]
    github_token: String,

    #[arg(short, long, default_value_t = ONE_DAY)]
    cache_seconds: u64,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv().ok();
        let args = Args::parse();
        let mut cfg_builder = config::Config::builder();
        cfg_builder = cfg_builder.add_source(
            Environment::default()
                .prefix("STATS")
                .prefix_separator("_")
                .separator("__"),
        );
        cfg_builder = cfg_builder.add_source(File::with_name(".env").required(false));
        if !args.github_token.is_empty() {
            cfg_builder = cfg_builder.set_override("github_token", args.github_token)?;
        }
        if args.cache_seconds != ONE_DAY {
            cfg_builder = cfg_builder.set_override("cache_seconds", args.cache_seconds)?;
        }
        let config = cfg_builder.build()?;
        config.try_deserialize()
    }
}

#[get("/")]
async fn root_endpoint() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/config")]
async fn config_endpoint(config: Data<Config>) -> impl Responder {
    HttpResponse::Ok().body(format!("{} {}", config.cache_seconds, config.github_token))
}

#[get("/cache")]
async fn cache_endpoint(config: Data<Config>) -> impl Responder {
    let now = SystemTime::now();
    HttpResponse::Ok()
        .insert_header((
            header::CACHE_CONTROL,
            format!(
                "public, immutable, max-age={}, s-maxage={}, stale-while-revalidate={}, stale-if-error={}",
                config.cache_seconds / 2,
                config.cache_seconds,
                ONE_DAY,
                ONE_DAY
            ),
        ))
        .body(format!("{:?}", now))
}

#[get("/stats")]
async fn stats_endpoint() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(header::ContentType(mime::IMAGE_SVG))
        .body("OK")
}

#[get("/top-langs")]
async fn toplangs_endpoint() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(header::ContentType(mime::IMAGE_SVG))
        .body("OK")
}

#[actix_web::main]
async fn main() -> Result<(), Error> {
    let config = match Config::from_env() {
        Ok(config) => config,
        Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err.to_string())),
    };

    crate::stats::test();
    crate::toplangs::test();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(config.clone()))
            .service(root_endpoint)
            .service(config_endpoint)
            .service(cache_endpoint)
            .service(stats_endpoint)
            .service(toplangs_endpoint)
    })
    .bind(("127.0.0.1", 8080))?
    .keep_alive(None)
    .shutdown_timeout(0)
    .run()
    .await
}
