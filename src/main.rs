use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::http::header::LOCATION;
use actix_web::middleware::Logger;
use env_logger::Env;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello there!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
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
            .service(hello)
            .service(echo)
            .service(redirect)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}