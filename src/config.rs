use serde::Deserialize;
use std::fs;
use std::path::Path;

// struct to hold config data - includes the Deserialize trait to easily deserialize the config
#[derive(Debug, Deserialize)]
pub struct Config {
    pub ext: Option<Vec<String>>,
    pub max_size: Option<u64>,
    pub format: Option<String>,
    pub report_format: Option<String>,
    pub signing_alg: Option<String>,
}

impl Config {
    // function to deserialize content - P is any type that converts to Path
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(&path)?;
        if path.as_ref().extension().and_then(|s| s.to_str()) == Some("json") {
            Ok(serde_json::from_str(&content)?)
        } else {
            Ok(serde_yaml::from_str(&content)?)
        }
    }
}