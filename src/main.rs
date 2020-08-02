#[macro_use]
extern crate lazy_static;

mod handler;
mod config;
mod object;
mod cache;

use actix_web::{web, App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    lazy_static::initialize(&handler::SETTINGS);
    HttpServer::new(|| {
        App::new()
            .route("{namespace}/{path}", web::get().to(handler::handle))
    })
        .bind("0.0.0.0:8088")?
        .run()
        .await
}