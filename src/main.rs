use std::{
    fs,
    io::{self, Error},
    path::Path,
    time::{Duration, SystemTime},
};

use actix_files::NamedFile;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::http::{Method, StatusCode};
use actix_web::{
    get, post,
    http::header::{self, CacheControl, CacheDirective},
    middleware::Logger,
    web,
    web::Data,
    App, Either, HttpResponse, HttpServer, Responder,
};
use config::{ConfigError, Environment};

use env_logger::Target;
use log::{debug, error, info, LevelFilter};
use serde::{Deserialize, Serialize};

mod card;
mod github;
mod icons;
mod language_colors;
mod stats;
mod themes;
mod toplangs;

const ONE_DAY: u32 = 86400;
const STATS_CACHE_JSON: &str = "stats_cache.json";

#[derive(Deserialize, Clone)]
struct Config {
    cache_seconds: u32,
    github_user: String,
    github_token: String,
    base_url: String,
    cache_path: String,
    ipv4_address: String,
    ipv6_address: String,
    port: u16,
    ignored_repositories: String,
    exclude_forks: bool,
    /// Empty string means the /refresh endpoint is disabled.
    refresh_token: String,
}

impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("cache_seconds", &self.cache_seconds)
            .field("github_user", &self.github_user)
            .field("github_token", &"[REDACTED]")
            .field("base_url", &self.base_url)
            .field("cache_path", &self.cache_path)
            .field("ipv4_address", &self.ipv4_address)
            .field("ipv6_address", &self.ipv6_address)
            .field("port", &self.port)
            .field("ignored_repositories", &self.ignored_repositories)
            .field("exclude_forks", &self.exclude_forks)
            .field(
                "refresh_token",
                if self.refresh_token.is_empty() { &"(not set)" } else { &"[REDACTED]" },
            )
            .finish()
    }
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        config::Config::builder()
            .set_default("cache_seconds", ONE_DAY)?
            .set_default("base_url", "")?
            .set_default("cache_path", "")?
            .set_default("ipv4_address", "0.0.0.0")?
            .set_default("ipv6_address", "")?
            .set_default("port", 8080)?
            .set_default("ignored_repositories", "")?
            .set_default("exclude_forks", false)?
            .set_default("refresh_token", "")?
            .add_source(
                Environment::default()
                    .prefix("STATS")
                    .prefix_separator("_")
                    .separator("__"),
            )
            .build()?
            .try_deserialize()
    }
}

fn stats_cache_path(cache_path: &str) -> std::path::PathBuf {
    Path::join(Path::new(cache_path), STATS_CACHE_JSON)
}

fn read_cached_stats(
    path: &Path,
    max_age_secs: u32,
) -> Result<Option<github::Stats>, anyhow::Error> {
    if !path.exists() {
        return Ok(None);
    }
    let threshold = SystemTime::now() - Duration::from_secs(max_age_secs.into());
    if let Ok(modified) = fs::metadata(path)?.modified() {
        if modified > threshold {
            let stats = serde_json::from_str(&fs::read_to_string(path)?)?;
            return Ok(Some(stats));
        }
    }
    Ok(None)
}

fn write_cached_stats(path: &Path, stats: &github::Stats) -> Result<(), anyhow::Error> {
    fs::write(path, serde_json::to_string(stats)?.as_bytes())?;
    Ok(())
}

async fn fetch_stats(config: &Config) -> Result<github::Stats, anyhow::Error> {
    let path = stats_cache_path(&config.cache_path);
    match read_cached_stats(&path, config.cache_seconds) {
        Ok(Some(stats)) => {
            debug!("Serving stats from cache");
            return Ok(stats);
        }
        Ok(None) => {}
        Err(err) => debug!("Stats cache unreadable: {err}"),
    }

    let fetched = github::Stats::request(
        &config.github_user,
        &config.github_token,
        &config.ignored_repositories,
        config.exclude_forks,
    )
    .await?;

    if let Err(err) = write_cached_stats(&path, &fetched) {
        debug!("Failed to write stats cache: {err}");
    }

    Ok(fetched)
}

fn svg_response(svg: String, cache_seconds: u32) -> HttpResponse {
    HttpResponse::Ok()
        .insert_header(header::ContentType(mime::IMAGE_SVG))
        .insert_header(("X-Content-Type-Options", "nosniff"))
        .insert_header(CacheControl(vec![
            CacheDirective::Public,
            CacheDirective::MaxAge(cache_seconds / 2),
            CacheDirective::SMaxAge(cache_seconds),
            CacheDirective::Extension(
                String::from("stale-while-revalidate"),
                Some(format!("{ONE_DAY}")),
            ),
        ]))
        .body(svg)
}

fn error_svg_response(message: &str, query: &CardQuery) -> HttpResponse {
    HttpResponse::Ok()
        .insert_header(header::ContentType(mime::IMAGE_SVG))
        .insert_header(("X-Content-Type-Options", "nosniff"))
        .body(card::render_error_card(
            message,
            query.width(),
            &query.theme(),
        ))
}

fn json_response(json: String) -> HttpResponse {
    HttpResponse::Ok()
        .insert_header(header::ContentType(mime::APPLICATION_JSON))
        .insert_header(("X-Content-Type-Options", "nosniff"))
        .body(json)
}

#[derive(Deserialize, Default)]
struct CardQuery {
    theme: Option<String>,
    lang_count: Option<usize>,
    width: Option<f64>,
}

impl CardQuery {
    fn theme(&self) -> themes::Theme {
        themes::from_name(self.theme.as_deref().unwrap_or("dark"))
    }

    fn width(&self) -> f64 {
        self.width.map(|w| w.clamp(50.0, 2000.0)).unwrap_or(300.0)
    }

    fn lang_count(&self) -> usize {
        self.lang_count.map(|c| c.clamp(1, 100)).unwrap_or(10)
    }
}

#[derive(Serialize)]
struct Endpoints {
    root_url: String,
    stats_url: String,
    languages_url: String,
    combined_url: String,
    refresh_url: String,
    health_url: String,
    favicon_url: String,
}

#[get("/")]
async fn root_endpoint(config: Data<Config>) -> Result<HttpResponse, Error> {
    let endpoints = Endpoints {
        root_url: format!("{}/", config.base_url),
        stats_url: format!("{}/stats", config.base_url),
        languages_url: format!("{}/languages", config.base_url),
        combined_url: format!("{}/combined", config.base_url),
        refresh_url: format!("{}/refresh", config.base_url),
        health_url: format!("{}/health", config.base_url),
        favicon_url: format!("{}/favicon.ico", config.base_url),
    };
    Ok(json_response(serde_json::to_string_pretty(&endpoints)?))
}

#[get("/stats")]
async fn stats_endpoint(
    config: Data<Config>,
    query: web::Query<CardQuery>,
) -> impl Responder {
    match build_stats_svg(&config, &query).await {
        Ok(svg) => svg_response(svg, config.cache_seconds),
        Err(err) => {
            error!("Failed to render /stats card: {err:#}");
            error_svg_response("Failed to fetch GitHub stats", &query)
        }
    }
}

async fn build_stats_svg(config: &Config, query: &CardQuery) -> Result<String, anyhow::Error> {
    let card_stats = fetch_stats(config).await?;
    let theme = query.theme();
    let width = query.width();
    let x_offset = 25.0_f64;
    let y_offset = 20.0_f64;
    let gap = 30.0_f64;
    let content_width = width - 2.0 * x_offset;

    let rendered_stats =
        stats::render_stats(&theme, card_stats.total_stars, card_stats.total_commits, content_width);

    Ok(card::render_card(
        vec![rendered_stats],
        x_offset,
        y_offset,
        gap,
        width,
        "Stats",
        &theme,
    ))
}

#[get("/combined")]
async fn combined_endpoint(
    config: Data<Config>,
    query: web::Query<CardQuery>,
) -> impl Responder {
    match build_combined_svg(&config, &query).await {
        Ok(svg) => svg_response(svg, config.cache_seconds),
        Err(err) => {
            error!("Failed to render /combined card: {err:#}");
            error_svg_response("Failed to fetch GitHub stats", &query)
        }
    }
}

async fn build_combined_svg(config: &Config, query: &CardQuery) -> Result<String, anyhow::Error> {
    let card_stats = fetch_stats(config).await?;
    let theme = query.theme();
    let width = query.width();
    let x_offset = 25.0_f64;
    let y_offset = 20.0_f64;
    let gap = 30.0_f64;
    let content_width = width - 2.0 * x_offset;

    let rendered_stats =
        stats::render_stats(&theme, card_stats.total_stars, card_stats.total_commits, content_width);
    let rendered_langs = toplangs::render_top_languages(
        &theme,
        x_offset,
        width,
        &card_stats.languages,
        query.lang_count(),
    );

    Ok(card::render_card(
        vec![rendered_stats, rendered_langs],
        x_offset,
        y_offset,
        gap,
        width,
        "Stats",
        &theme,
    ))
}

#[get("/languages")]
async fn languages_endpoint(
    config: Data<Config>,
    query: web::Query<CardQuery>,
) -> impl Responder {
    match build_languages_svg(&config, &query).await {
        Ok(svg) => svg_response(svg, config.cache_seconds),
        Err(err) => {
            error!("Failed to render /languages card: {err:#}");
            error_svg_response("Failed to fetch GitHub languages", &query)
        }
    }
}

async fn build_languages_svg(
    config: &Config,
    query: &CardQuery,
) -> Result<String, anyhow::Error> {
    let card_stats = fetch_stats(config).await?;
    let theme = query.theme();
    let width = query.width();
    let x_offset = 25.0_f64;
    let y_offset = 20.0_f64;
    let gap = 30.0_f64;

    let rendered_langs = toplangs::render_top_languages(
        &theme,
        x_offset,
        width,
        &card_stats.languages,
        query.lang_count(),
    );

    Ok(card::render_card(
        vec![rendered_langs],
        x_offset,
        y_offset,
        gap,
        width,
        "Top Languages",
        &theme,
    ))
}

#[post("/refresh")]
async fn refresh_endpoint(config: Data<Config>, req: actix_web::HttpRequest) -> impl Responder {
    if config.refresh_token.is_empty() {
        return HttpResponse::Forbidden()
            .body("Refresh endpoint disabled: set STATS_REFRESH_TOKEN to enable");
    }

    let provided = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .unwrap_or("");

    if provided != config.refresh_token {
        return HttpResponse::Unauthorized()
            .insert_header(("WWW-Authenticate", "Bearer"))
            .finish();
    }

    let path = stats_cache_path(&config.cache_path);
    match fs::remove_file(&path) {
        Ok(_) => {
            info!("Cache invalidated: {}", path.display());
            HttpResponse::Ok().body("Cache cleared")
        }
        Err(err) if err.kind() == io::ErrorKind::NotFound => {
            HttpResponse::Ok().body("No cache to clear")
        }
        Err(err) => {
            error!("Failed to clear cache: {err}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    cache: CacheStatus,
}

#[derive(Serialize)]
struct CacheStatus {
    exists: bool,
    age_seconds: Option<u64>,
    fresh: bool,
}

fn cache_status(path: &Path, max_age_secs: u32) -> CacheStatus {
    if !path.exists() {
        return CacheStatus { exists: false, age_seconds: None, fresh: false };
    }
    if let Ok(metadata) = fs::metadata(path) {
        if let Ok(modified) = metadata.modified() {
            if let Ok(elapsed) = SystemTime::now().duration_since(modified) {
                let age = elapsed.as_secs();
                return CacheStatus {
                    exists: true,
                    age_seconds: Some(age),
                    fresh: age < max_age_secs as u64,
                };
            }
        }
    }
    CacheStatus { exists: true, age_seconds: None, fresh: false }
}

#[get("/health")]
async fn health_endpoint(config: Data<Config>) -> impl Responder {
    let cache = cache_status(&stats_cache_path(&config.cache_path), config.cache_seconds);
    let (http_status, status_text) = if cache.fresh {
        (StatusCode::OK, "healthy")
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, "degraded")
    };
    let health = HealthResponse {
        status: status_text,
        cache,
    };
    let json = serde_json::to_string(&health).expect("HealthResponse is always serializable");
    HttpResponse::build(http_status)
        .insert_header(header::ContentType(mime::APPLICATION_JSON))
        .insert_header(("X-Content-Type-Options", "nosniff"))
        .body(json)
}

#[get("/favicon.ico")]
async fn favicon_endpoint() -> Result<impl Responder, Error> {
    NamedFile::open("static/favicon.ico")
}

async fn default_handler(req_method: Method) -> Result<impl Responder, Error> {
    match req_method {
        Method::GET => {
            let file = NamedFile::open("static/404.html")?
                .customize()
                .with_status(StatusCode::NOT_FOUND);
            Ok(Either::Left(file))
        }
        _ => Ok(Either::Right(HttpResponse::MethodNotAllowed().finish())),
    }
}

/// Loads the .env file before the async runtime starts.
///
/// `std::env::set_var` is not thread-safe, so it must be called from a
/// single-threaded context. A missing .env file is silently ignored
/// (production deployments pass vars through the environment directly);
/// any other error — such as a malformed file — is printed to stderr.
fn load_dotenv() {
    match dotenvy::dotenv() {
        Ok(_) => {}
        Err(dotenvy::Error::Io(ref e)) if e.kind() == io::ErrorKind::NotFound => {}
        Err(e) => eprintln!("Warning: failed to load .env: {e}"),
    }
}

fn main() -> Result<(), Error> {
    load_dotenv();
    actix_web::rt::System::new().block_on(run())
}

async fn run() -> Result<(), Error> {
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .parse_default_env()
        .target(Target::Stdout)
        .init();

    let config = match Config::from_env() {
        Ok(config) => config,
        Err(err) => {
            error!("Failed to read config: {:?}", err);
            return Err(Error::other(err.to_string()));
        }
    };

    let ipv4_address = config.ipv4_address.clone();
    let ipv6_address = config.ipv6_address.clone();
    let port = config.port;

    let governor_conf = match GovernorConfigBuilder::default()
        .seconds_per_request(3)
        .burst_size(3)
        .finish()
    {
        Some(conf) => conf,
        None => {
            error!("Failed to build Governor config");
            return Err(Error::other("Failed to build Governor config"));
        }
    };

    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Governor::new(&governor_conf))
            .app_data(Data::new(config.clone()))
            .service(root_endpoint)
            .service(stats_endpoint)
            .service(languages_endpoint)
            .service(combined_endpoint)
            .service(refresh_endpoint)
            .service(health_endpoint)
            .service(favicon_endpoint)
            .default_service(web::to(default_handler))
    });

    let mut has_binding = false;

    if !ipv4_address.is_empty() {
        match server.bind((ipv4_address.clone(), port)) {
            Ok(s) => {
                info!("Listening on {ipv4_address}:{port}");
                server = s;
                has_binding = true;
            }
            Err(err) => {
                error!("Failed to bind to {ipv4_address}:{port}: {err:?}");
                return Err(Error::other(err.to_string()));
            }
        }
    }

    if !ipv6_address.is_empty() {
        match server.bind((ipv6_address.clone(), port)) {
            Ok(s) => {
                info!("Listening on {ipv6_address}:{port}");
                server = s;
                has_binding = true;
            }
            Err(err) => {
                error!("Failed to bind to {ipv6_address}:{port}: {err:?}");
                return Err(Error::other(err.to_string()));
            }
        }
    }

    if !has_binding {
        error!("No valid IP address configured");
        return Err(Error::other("No valid IP address configured"));
    }

    server.workers(2).run().await
}
