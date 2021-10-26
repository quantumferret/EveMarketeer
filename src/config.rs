use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub discord_token: String,
    pub base_url: String,
    pub callback_url: String,
    pub port: u16,
    pub discord_client_id: u64,
    pub discord_client_secret: String,
    pub discord_public_key: String,
    pub bot_permissions: u64,
    pub esi_client_id: String,
    pub esi_secret_key: String,
}

impl Config {
    pub fn new_from_ron_file(config_path: String) -> Self {
        let f = File::open(&config_path).expect("Failed to open file");
        let config: Config = match from_reader(f) {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to open configuration file: {}", e);
                std::process::exit(1);
            }
        };
        config
    }
}

