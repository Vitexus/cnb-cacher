// src/main.rs

use actix_web::{web, App, HttpServer};
use crate::routes::init_routes;

mod routes;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(init_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}