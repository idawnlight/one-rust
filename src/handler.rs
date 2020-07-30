use actix_web::HttpRequest;
use crate::object::{Object, Resp, Data};
use crate::config::Config;
use std::collections::HashMap;
use actix_web::http::StatusCode;
use sha2::{Sha256, Digest};
use std::thread;
use chrono::Utc;
use crate::cache::refresh_cache;

lazy_static! {
    pub static ref SETTINGS: HashMap<String, Config> = {
        crate::config::init().unwrap()
    };
}

pub async fn handle(req: HttpRequest) -> Resp {
    if req.match_info().get("namespace").is_none() || req.match_info().get("path").is_none() {
        return Resp {
            object: Object { data: Data { content: "bad request".to_owned(), ..Default::default() }, ..Default::default() },
            http_status: StatusCode::BAD_REQUEST,
            ..Default::default()
        };
    }
    let namespace = req.match_info().get("namespace").unwrap();
    let path = req.match_info().get("path").unwrap();
    if !SETTINGS.contains_key(namespace) {
        return Resp {
            object: Object { data: Data { content: "namespace not found".to_owned(), ..Default::default() }, ..Default::default() },
            http_status: StatusCode::NOT_FOUND,
            ..Default::default()
        };
    }
    let config = SETTINGS.get(namespace).unwrap();
    let identifier = hex::encode(Sha256::digest((namespace.to_owned() + path).as_ref()));
    let uri = (&config.host).to_owned() + path;
    // println!("{:?}", identifier);
    let object = crate::cache::from_cache(identifier.clone());
    // println!("{:?}", object);
    match object {
        Some(mut o) => {
            o.data = Data {
                content: crate::cache::get_data(&identifier),
                ..Default::default()
            };
            if Utc::now().timestamp() - o.date.timestamp() > config.expiration {
                thread::spawn(move || {
                    refresh_cache(&identifier, uri)
                });
            }
            Resp { object: o, ..Default::default()}
        },
        None => {
            thread::spawn(move || {
                refresh_cache(&identifier, uri)
            });
            redirect((&config.host).to_owned() + path)
        }
    }
    // Resp {
    //     object: Object {
    //         content: identifier + &*uri,
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // }
}

fn redirect(uri: String) -> Resp {
    let mut headers = HashMap::new();
    headers.insert("Location".to_owned(), uri);
    Resp {
        http_status: StatusCode::TEMPORARY_REDIRECT,
        extra_headers: headers,
        ..Default::default()
    }
}