use actix_web::HttpRequest;
use crate::object::{Object, Resp};
use crate::config::Config;
use std::collections::HashMap;
use actix_web::http::StatusCode;
use sha2::{Sha256, Digest};

lazy_static! {
    pub static ref SETTINGS: HashMap<String, Config> = {
        crate::config::init().unwrap()
    };
}

pub async fn handle(req: HttpRequest) -> Resp {
    if req.match_info().get("namespace").is_none() || req.match_info().get("path").is_none() {
        return Resp {
            object: Object { content: "bad request".to_owned(), ..Default::default() },
            http_status: StatusCode::BAD_REQUEST,
            ..Default::default()
        };
    }
    let namespace = req.match_info().get("namespace").unwrap();
    let path = req.match_info().get("path").unwrap();
    if !SETTINGS.contains_key(namespace) {
        return Resp {
            object: Object { content: "namespace not found".to_owned(), ..Default::default() },
            http_status: StatusCode::NOT_FOUND,
            ..Default::default()
        };
    }
    let config = SETTINGS.get(namespace).unwrap();
    let identifier = hex::encode(Sha256::digest((namespace.to_owned() + path).as_ref()));
    let uri = (&config.host).to_owned() + path;
    Resp {
        object: Object {
            content: identifier + &*uri,
            ..Default::default()
        },
        ..Default::default()
    }
}