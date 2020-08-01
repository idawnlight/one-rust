#[macro_use]
extern crate lazy_static;

mod handler;
mod config;
mod object;
mod cache;

use actix_web::{web, App, HttpServer};
use object::Object;
use crate::object::Resp;

async fn index() -> Resp {
    Resp {
        object: Object {
            data: object::Data {
                content: "hello word by idawnlight/one-rust".to_string().into_bytes(),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    lazy_static::initialize(&handler::SETTINGS);
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("{namespace}/{path}", web::get().to(handler::handle))
    })
        .workers(num_cpus::get())
        .bind("127.0.0.1:8088")?
        .run()
        .await
}