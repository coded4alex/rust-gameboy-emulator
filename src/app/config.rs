use std::fs;

use serde::{Serialize, Deserialize};
use serde_yaml;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
}


impl Config {
    pub fn new(path: String) -> Config {
        let config_file = fs::read_to_string(path).expect("Error reading config file");
        serde_yaml::from_str(&config_file).expect("Error parsing config file")
    }
}