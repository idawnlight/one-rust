use std::error::Error;
use std::path::Path;
use std::fs::{read_to_string, read_dir};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub host: String,
    pub expiration: u32,
}

pub fn read(namespace: String) -> Result<Config, Box<dyn Error>> {
    let rpath = "config/".to_owned() + &namespace + ".json";
    let path = Path::new(&rpath);
    if !path.exists() {
        Err("Config of namespace ".to_owned() + &namespace + " not found.")?
    }
    match serde_json::from_str::<Config>(&*read_to_string(path).unwrap()) {
        Ok(c) => Ok(c),
        Err(error) => Err("Fail to parse ".to_owned() + &rpath + ": " + &error.to_string())?,
    }
}

pub fn init() -> Result<HashMap<String, Config>, Box<dyn Error>> {
    let mut config = HashMap::new();
    let entries = read_dir("config/")?
        .map(|res| res.map(|e| e.file_name()))
        .collect::<Result<Vec<_>, std::io::Error>>()?;
    for entry in entries {
        let namespace: String = entry.to_str().unwrap().to_string().split(".json").collect::<Vec<_>>().first().unwrap().to_string();
        let res = read(namespace.to_owned())?;
        config.insert(namespace, res);
    }
    Ok(config)
}