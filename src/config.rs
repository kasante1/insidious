use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub excluded_dirs: Vec<String>,
    pub file_extensions: Vec<String>,
    pub similarity_threshold: f64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            excluded_dirs: vec!["node_modules".to_string(), "dist".to_string(), "build".to_string()],
            file_extensions: vec!["js".to_string(), "ts".to_string()],
            similarity_threshold: 70.0,
        }
    }
}

pub fn init_config() {
    let config = Config::default();
    let json = serde_json::to_string_pretty(&config).unwrap();
    fs::write(".express-analyzer.json", json).unwrap();
}

pub fn load_config() -> Config {
    match fs::read_to_string(".express-analyzer.json") {
        Ok(json) => {
            match serde_json::from_str(&json) {
                Ok(config) => config,
                Err(_) => Config::default(),
            }
        },
        Err(_) => Config::default(),
    }
}