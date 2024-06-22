use std::{env, io};
use std::io::ErrorKind;
use std::num::ParseIntError;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

fn read_cache_seconds() -> Result<u64, io::Error> {
    let key = "CACHE_SECONDS";
    match env::var(key) {
        Ok(val) => val.parse::<u64>().map_err(|e: ParseIntError| io::Error::new(ErrorKind::InvalidData, e.to_string())),
        Err(e) => Err(io::Error::new(ErrorKind::NotFound, format!("Environment variable {key} not found: {e}"))),
    }
}

fn read_github_token() -> Result<String, io::Error> {
    let key = "GITHUB_TOKEN";
    match env::var(key) {
        Ok(val) => Ok(val),
        Err(e) => Err(io::Error::new(ErrorKind::NotFound, format!("Environment variable {key} not found: {e}"))),
    }
}

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::NotFound()
}

#[get("/stats")]
async fn stats() -> impl Responder {
    let cache_seconds = read_cache_seconds();
    if let Err(e) = cache_seconds {
        return HttpResponse::InternalServerError().body(e.to_string());
    }
    
    let github_token = read_github_token();
    if let Err(e) = github_token {
        return HttpResponse::InternalServerError().body(e.to_string());
    }
    
    HttpResponse::Ok().body(format!("CACHE_SECONDS: {}\nGITHUB_TOKEN: {}", cache_seconds.unwrap(), github_token.unwrap()))
}

#[get("/top-langs")]
async fn toplangs() -> impl Responder {
    let cache_seconds = read_cache_seconds();
    if let Err(e) = cache_seconds {
        return HttpResponse::InternalServerError().body(e.to_string());
    }

    let github_token = read_github_token();
    if let Err(e) = github_token {
        return HttpResponse::InternalServerError().body(e.to_string());
    }

    HttpResponse::Ok().body(format!("CACHE_SECONDS: {}\nGITHUB_TOKEN: {}", cache_seconds.unwrap(), github_token.unwrap()))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(root)
            .service(stats)
            .service(toplangs)
    })
        .bind(("127.0.0.1", 8080))?
        .keep_alive(None)
        .shutdown_timeout(0)
        .run()
        .await
}
