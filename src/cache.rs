use crate::object::{Object, ObjectType};
use std::path::Path;
use std::fs;
use chrono::Utc;
use reqwest::blocking::Response;

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

pub fn get_data(identifier: &str) -> Vec<u8> {
    let path = "cache/".to_owned() + identifier + ".data";
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

    // let mut ret_vec: [u8];

    // let mut gz = GzEncoder::new(&data.into_bytes(), Compression::default());
    // let count = gz.read(&mut ret_vec)?;
    // let data_gz = hex::encode(ret_vec[0..count].to_vec());
    //
    // let mut params = BrotliEncoderParams::default();
    // // modify params to fit the application needs
    // let mut writer = brotli::CompressorReader::with_params(&data.into_bytes(), 4096 /* buffer size */, &params);

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

    fs::write("cache/".to_owned() + &identifier + ".data", data)?;
    // fs::write("cache/".to_owned() + &identifier + ".data.gz", data_gz)?;
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