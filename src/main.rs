mod models;
mod routes;
mod services;
mod utils;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::io::Result;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello there")
}

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(move || App::new().service(hello))
        .bind("localhost:5005")
        .unwrap()
        .run()
        .await
}
