use actix::prelude::*;
use actix_cors::Cors;
use actix_redis::{Command, RedisActor, RespValue};
use actix_web::http::header::LOCATION;
use actix_web::middleware::Logger;
use actix_web::web::{Data, Json};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use redis_async::resp_array;
use serde::{Deserialize, Serialize};

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
async fn clip(req_body: Json<Url>, redis: Data<Addr<RedisActor>>) -> impl Responder {
    let redis_command = resp_array!["SET", "key_name", "123"];
    let redis_result = redis.send(Command(redis_command)).await;
    if let Ok(Ok(RespValue::SimpleString(x))) = redis_result {
        println!("{}", x);
    } else {
        println!("Error");
    }
    HttpResponse::Ok().json(Url {
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
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .data(RedisActor::start("127.0.0.1:6379"))
            .service(hello)
            .service(echo)
            .service(clip)
            .service(redirect)
    })
    .bind("127.0.0.1:8090")?
    .run()
    .await
}
