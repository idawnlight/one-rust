use actix_web::{Responder, Error, HttpRequest, HttpResponse};
use futures::future::{ready, Ready};
use serde::{Serialize, Deserialize};
use actix_web::http::StatusCode;
use chrono::{Utc, DateTime};
use chrono::serde::ts_seconds;
use std::collections::HashMap;
use crate::config::Config;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ObjectType {
    Binary,
    Image
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ContentEncoding {
    Br,
    Deflate,
    Gzip,
    Identity,
}

impl ToString for ContentEncoding {
    fn to_string(&self) -> String {
        match self {
            ContentEncoding::Br => "br".to_string(),
            ContentEncoding::Deflate => "deflate".to_string(),
            ContentEncoding::Gzip => "gzip".to_string(),
            _ => "identity".to_string()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    pub content: Vec<u8>,
    pub content_encoding: ContentEncoding,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Object {
    pub identifier: String,
    pub object_type: ObjectType,
    pub content_type: String,
    #[serde(with = "ts_seconds")]
    pub date: DateTime<Utc>,
    pub with_data: bool,
    pub data: Data
}

impl Default for Data {
    fn default() -> Data {
        Data {
            content: Vec::new(),
            content_encoding: ContentEncoding::Identity,
        }
    }
}

impl Default for Object {
    fn default() -> Object {
        Object {
            identifier: "default".to_owned(),
            object_type: ObjectType::Binary,
            content_type: "text/html; charset=UTF-8".to_owned(),
            date: Utc::now(),
            with_data: false,
            data: Data { ..Default::default() }
        }
    }
}

pub struct Resp {
    pub object: Object,
    pub config: &'static Config,
    pub http_status: StatusCode,
    pub extra_headers: HashMap<String, String>
}

impl Default for Resp {
    fn default() -> Resp {
        Resp {
            object: Object { ..Default::default() },
            config: crate::handler::SETTINGS.get("__empty").unwrap(),
            http_status: StatusCode::OK,
            extra_headers: HashMap::new()
        }
    }
}

impl Resp {
    fn is_expired(&self) -> bool {
        Utc::now().timestamp() - self.object.date.timestamp() > self.config.expiration
    }
}

impl Responder for Resp {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        ready(Ok(
            {
                let mut response = HttpResponse::build(self.http_status);
                response
                    .content_type(&self.object.content_type)
                    .header("content-encoding", self.object.data.content_encoding.to_string())
                    .header("ETag", self.object.date.timestamp().to_string() + "-" + &self.object.data.content.len().to_string())
                    .header("x-powered-by", "idawnlight/one-rust");
                match &self.is_expired() {
                    true => response.header("cache-control", "no-cache, max-age=0"),
                    false => response.header("cache-control", "max-age=".to_owned() + &*self.config.expiration.to_string())
                };
                for header in self.extra_headers {
                    response.header(&*header.0, &*header.1);
                }
                response.body(self.object.data.content)
            }
        ))
    }
}