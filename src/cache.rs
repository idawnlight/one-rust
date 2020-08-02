use crate::object::{Object, ObjectType, ContentEncoding};
use std::path::Path;
use std::fs;
use chrono::Utc;
use reqwest::blocking::Response;
use flate2::{GzBuilder, Compression};
use std::io::Write;
use brotli::enc::BrotliEncoderParams;

pub fn from_cache(identifier: String) -> Option<Object> {
    let path = "cache/".to_owned() + &identifier + ".meta";
    let path = Path::new(&path);
    if !path.exists() {
        return None;
    }
    match serde_json::from_str::<Object>(&*fs::read_to_string(path).unwrap()) {
        Ok(c) => Some(c),
        _ => None
    }
}

pub fn get_data(identifier: &str, encoding: ContentEncoding) -> Vec<u8> {
    let mut path = "cache/".to_owned() + identifier + ".data";
    match encoding {
        ContentEncoding::Gzip => path += ".gz",
        ContentEncoding::Br => path += ".br",
        _ => {}
    }
    match std::fs::read(Path::new(&path)) {
        Ok(v) => v,
        Err(_) => Vec::new()
    }
}

pub fn refresh_cache(identifier: &str, uri: String) -> std::io::Result<()> {
    let resp = match get_source(uri) {
        Some(o) => o,
        _ => return Ok(())
    };
    let mut object = Object {
        identifier: identifier.to_owned(),
        content_type: resp.headers().get(reqwest::header::CONTENT_TYPE).unwrap().to_str().unwrap().to_string(),
        date: Utc::now(),
        ..Default::default()
    };

    match &resp.headers().get(reqwest::header::CONTENT_TYPE) {
        Some(c) => {
            match c.to_str().unwrap() {
                "image/jpeg" | "image/png" | "image/bmp" | "image/webp" | "image/gif" => { object.object_type = ObjectType::Image },
                _ => {}
            }
        },
        _ => {}
    }

    let data = &resp.bytes().unwrap();

    let mut data_gz = Vec::new();
    let mut gz = GzBuilder::new().write(&mut data_gz, Compression::default());
    gz.write_all(data)?;
    gz.finish()?;

    let mut data_br = Vec::new();
    let params = BrotliEncoderParams::default();
    let mut br = brotli::CompressorWriter::with_params(&mut data_br, 4096, &params);
    br.write_all(data)?;
    br.flush()?;
    std::mem::drop(br);

    fs::write("cache/".to_owned() + &identifier + ".data", data)?;
    fs::write("cache/".to_owned() + &identifier + ".data.gz", data_gz)?;
    fs::write("cache/".to_owned() + &identifier + ".data.br", data_br)?;
    fs::write("cache/".to_owned() + &identifier + ".meta", serde_json::to_string(&object).unwrap())
}

fn get_source(uri: String) -> Option<Response> {
    let resp = match reqwest::blocking::get(&uri) {
        Ok(r) => r,
        _ => return None
    };

    if resp.status() != reqwest::StatusCode::OK { return None }

    Some(resp)
}