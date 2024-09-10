mod models;
mod routes;
mod services;
mod utils;

use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
use routes::user_router::{
    delete_user_by_id, get_all_users, get_user_by_id, insert_new_user, update_user_by_id,
};
use std::io::Result;

use services::db::Storage;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello there")
}

#[actix_web::main]
async fn main() -> Result<()> {
    let storage = Storage::init().await.unwrap();
    let storage_data = Data::new(storage);

    HttpServer::new(move || {
        App::new()
            .service(hello)
            .service(get_all_users)
            .service(get_user_by_id)
            .service(insert_new_user)
            .service(update_user_by_id)
            .service(delete_user_by_id)
            .app_data(storage_data.clone())
    })
    .bind("localhost:5005")
    .unwrap()
    .run()
    .await
}
