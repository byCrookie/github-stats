use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{
    get,
    http::header::{self, CacheControl, CacheDirective},
    middleware::Logger,
    web::Data,
    App, HttpResponse, HttpServer, Responder,
};
use config::{ConfigError, Environment};
use dotenv::dotenv;
use env_logger::Target;
use log::{debug, info, LevelFilter};
use mime;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    fs,
    io::{self, Error, Read, Write},
    path::Path,
    time::{Duration, SystemTime},
};
use themes::Theme;

mod card;
mod github;
mod icons;
mod languagecolors;
mod stats;
mod themes;
mod toplangs;

const ONE_DAY: u32 = 86400;
const ALL_CACHE_SVG: &str = "all_cache.svg";

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Config {
    cache_seconds: u32,
    github_user: String,
    github_token: String,
    base_url: String,
    cache_path: String,
    address: String,
    port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg_builder = config::Config::builder();
        cfg_builder = cfg_builder.set_default("cache_seconds", ONE_DAY)?;
        cfg_builder = cfg_builder.set_default("base_url", "")?;
        cfg_builder = cfg_builder.set_default("cache_path", "")?;
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

fn read_file_if_recent(path: &Path, seconds: u32) -> Result<Option<String>, anyhow::Error> {
    if !path.exists() {
        return Ok(None);
    }

    let now = SystemTime::now();
    let threshold = now - Duration::from_secs(seconds.into());
    let metadata = fs::metadata(path)?;

    if let Ok(modified) = metadata.modified() {
        if modified > threshold {
            let mut file = fs::File::open(path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            return Ok(Some(contents));
        }
    }

    Ok(None)
}

#[derive(Serialize)]
struct Endpoints {
    all_url: String,
}

#[get("/")]
async fn root_endpoint(config: Data<Config>) -> impl Responder {
    let endpoints = Endpoints {
        all_url: format!("{}/all", config.base_url),
    };

    HttpResponse::Ok().body(format!("{}", json!(endpoints)))
}

#[get("/all")]
async fn all_endpoint(config: Data<Config>) -> impl Responder {
    let cache_file_path = Path::join(Path::new(&config.cache_path), ALL_CACHE_SVG);
    let file_result = read_file_if_recent(&cache_file_path, config.cache_seconds);

    if let Ok(file) = file_result {
        if let Some(content) = file {
            return HttpResponse::Ok()
                .insert_header(header::ContentType(mime::IMAGE_SVG))
                .body(content);
        }
    } else if let Err(err) = file_result {
        debug!("failed to read stats cache: {err}");
        return HttpResponse::InternalServerError().finish();
    }

    let stats = crate::github::request_stats(&config.github_user, &config.github_token).await;

    if let Err(_err) = stats {
        return HttpResponse::InternalServerError().finish();
    }

    if let Ok(stats) = stats {
        let file = std::fs::File::create(&cache_file_path);

        if let Err(err) = file {
            debug!("can not open/create {}: {}", cache_file_path.display(), err);
            return HttpResponse::InternalServerError().finish();
        }

        if let Ok(mut file) = file {
            let width: f64 = 300.0;
            let x_offset: f64 = 25.0;
            let y_offset: f64 = 35.0;
            let gap: f64 = 30.0;
            let lang_count: usize = 40;
            let title: &str = "Stats";
            let theme: Theme = crate::themes::dark();
            let rendered_stats =
                crate::stats::render_stats(&theme, stats.total_stars, stats.total_commits);
            let rendered_toplangs = crate::toplangs::render_top_languages(
                &theme,
                x_offset,
                width,
                stats.languages,
                lang_count,
            );
            let rendered_card = crate::card::render_card(
                vec![rendered_stats, rendered_toplangs],
                x_offset,
                y_offset,
                gap,
                width,
                title,
            );

            let write = file.write_all(rendered_card.as_bytes());
            if let Err(err) = write {
                debug!("can not write to {}: {}", cache_file_path.display(), err);
                return HttpResponse::InternalServerError().finish();
            }
            return HttpResponse::Ok()
                .insert_header(header::ContentType(mime::IMAGE_SVG))
                .insert_header(CacheControl(vec![
                    CacheDirective::Public,
                    CacheDirective::MaxAge(config.cache_seconds / 2),
                    CacheDirective::SMaxAge(config.cache_seconds),
                    CacheDirective::Extension(
                        String::from("stale-while-revalidate"),
                        Some(format!("{ONE_DAY}")),
                    ),
                ]))
                .body(rendered_card);
        }
    }

    return HttpResponse::InternalServerError().finish();
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
        Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err.to_string())),
    };

    // crate::card::test();
    // crate::stats::test();
    // crate::toplangs::test();
    // crate::github::test(&config.github_user, &config.github_token).await;

    let address: String = config.address.clone();
    let port: u16 = config.port.clone();

    let governor_conf = GovernorConfigBuilder::default()
        .per_second(5)
        .burst_size(3)
        .finish()
        .unwrap();

    info!("Running on {address}:{port}");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Governor::new(&governor_conf))
            .app_data(Data::new(config.clone()))
            .service(root_endpoint)
            .service(all_endpoint)
    })
    .bind((address, port))?
    .workers(2)
    .run()
    .await
}
