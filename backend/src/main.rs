use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::env;
use std::error::Error;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load env even if it fails
    let _ = dotenvy::dotenv();

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind((
        env::var("HOST").unwrap_or("127.0.0.1".to_string()),
        env::var("PORT")
            .unwrap_or("8080".to_string())
            .parse::<u16>()
            .expect("PORT must be a number"),
    ))?
    .run()
    .await
}
