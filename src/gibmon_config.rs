use std::fs;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    Spotify: SpotifyConfig,
}

impl Config {
    pub fn get_spotify_config(&mut self) -> &SpotifyConfig {
        &self.Spotify
    }
}

#[derive(Deserialize)]
pub struct SpotifyConfig {
    pub ClientId: String,
    pub ClientSecret: String,
}

pub fn load_config(file_path: &str) -> Config {
    let yaml_data = fs::read_to_string(file_path).expect("Unable to read YAML file");
    serde_yaml::from_str(&yaml_data).expect("Failed to parse YAML")
}
