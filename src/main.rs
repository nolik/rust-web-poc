use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::http::header::LOCATION;
use actix_web::middleware::Logger;
use env_logger::Env;
use serde::{Serialize, Deserialize};
use actix_web::http::header;
use actix_cors::Cors;

#[derive(Serialize, Deserialize)]
pub struct Url {
    address: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello there!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/clip")]
async fn clip(req_body: web::Json<Url>) -> impl Responder {
    HttpResponse::Ok()
        .json(Url {
            address: req_body.address.to_string(),
        })
}

#[get("/redirect/{url}")]
async fn redirect(web::Path(url): web::Path<String>) -> impl Responder {
    HttpResponse::PermanentRedirect()
        .header(LOCATION, "http://".to_owned() + &url)
        .finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .service(hello)
            .service(echo)
            .service(clip)
            .service(redirect)
    })
        .bind("127.0.0.1:8090")?
        .run()
        .await
}