use actix_web::{Responder, Error, HttpRequest, HttpResponse};
use futures::future::{ready, Ready};
use serde::{Serialize, Deserialize};
use actix_web::http::StatusCode;
use chrono::{Utc, DateTime};
use chrono::serde::ts_seconds;
use std::collections::HashMap;

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

impl ContentEncoding {
    fn to_string(&self) -> &str {
        match self {
            ContentEncoding::Br => "br",
            ContentEncoding::Deflate => "deflate",
            ContentEncoding::Gzip => "gzip",
            _ => "identity"
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    pub content: String,
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
            content: "".to_owned(),
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
    pub http_status: StatusCode,
    pub extra_headers: HashMap<String, String>
}

impl Default for Resp {
    fn default() -> Resp {
        Resp {
            object: Object { ..Default::default() },
            http_status: StatusCode::OK,
            extra_headers: HashMap::new()
        }
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
                    .content_type(self.object.content_type)
                    .header("content-encoding", self.object.data.content_encoding.to_string())
                    .header("x-powered-by", "idawnlight/one-rust");
                for header in self.extra_headers {
                    response.header(&*header.0, &*header.1);
                }
                response.body(self.object.data.content.to_owned())
            }
        ))
    }
}