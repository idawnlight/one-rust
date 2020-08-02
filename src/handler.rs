use actix_web::HttpRequest;
use crate::object::{Object, Resp, Data};
use crate::config::Config;
use std::collections::HashMap;
use actix_web::http::StatusCode;
use sha2::{Sha256, Digest};
use chrono::Utc;
use std::sync::RwLock;
use threadpool::ThreadPool;
use std::sync::Mutex;

lazy_static! {
    pub static ref SETTINGS: HashMap<String, Config> = {
        crate::config::init().unwrap()
    };

    static ref REFLOCK: RwLock<HashMap<String, bool>> = RwLock::new(HashMap::new());

    static ref THREADPOOL: Mutex<ThreadPool> = Mutex::new(ThreadPool::with_name("updater".to_string(), num_cpus::get()));
}

pub async fn handle(req: HttpRequest) -> Resp {
    if req.match_info().get("namespace").is_none() || req.match_info().get("path").is_none() {
        return Resp {
            object: Object { data: Data { content: "bad request".to_string().into_bytes(), ..Default::default() }, ..Default::default() },
            http_status: StatusCode::BAD_REQUEST,
            ..Default::default()
        };
    }
    let namespace = req.match_info().get("namespace").unwrap();
    let path = req.match_info().get("path").unwrap();
    if !SETTINGS.contains_key(namespace) {
        return Resp {
            object: Object { data: Data { content: "namespace not found".to_string().into_bytes(), ..Default::default() }, ..Default::default() },
            http_status: StatusCode::NOT_FOUND,
            ..Default::default()
        };
    }
    let config = SETTINGS.get(namespace).unwrap();
    let query_string = req.query_string();
    let identifier = hex::encode(Sha256::digest((namespace.to_owned() + path + "?" + query_string).as_ref()));
    let mut uri = (&config.host).to_owned() + path;
    if query_string.len() != 0 {
        uri = uri + "?" + query_string;
    }
    let object = crate::cache::from_cache(identifier.clone());
    match object {
        Some(mut o) => {
            o.data = Data {
                content: crate::cache::get_data(&identifier),
                ..Default::default()
            };
            if Utc::now().timestamp() - o.date.timestamp() > config.expiration && !REFLOCK.read().unwrap().contains_key(&identifier) {
                refresh(identifier.clone(), uri.clone());
            }
            Resp { object: o, config, ..Default::default() }
        },
        None => {
            refresh(identifier.clone(), uri.clone());
            redirect(uri)
        }
    }
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

fn refresh(identifier: String, uri: String) {
    THREADPOOL.lock().unwrap().execute(move || {
        REFLOCK.write().unwrap().insert(identifier.clone(), true);
        match crate::cache::refresh_cache(&identifier, uri) { _ => {} };
        REFLOCK.write().unwrap().remove(&identifier);
    });
}