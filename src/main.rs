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
    // let foo = Object {
    //     // content: "hello world".to_owned(),
    //     ..Default::default()
    // };
    Resp {
        object: Object {
            // content: serde_json::to_string(&foo).unwrap(),
            ..Default::default()
        },
        ..Default::default()
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // CONFIG = config::init().unwrap();
    lazy_static::initialize(&handler::SETTINGS);
    // print!("{:?}", (*handler::SETTINGS));
    // Ok(())
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("{namespace}/{path}", web::get().to(handler::handle))
    })
        // .workers(4) // <- Start 4 workers
        .bind("127.0.0.1:8088")?
        .run()
        .await
}