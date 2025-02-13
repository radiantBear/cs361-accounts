use std::sync::OnceLock;
use std::env;
use dotenvy::dotenv;

use crate::utils::rand;


pub static CONFIG: OnceLock<Config> = OnceLock::new();


pub struct Config {
    pub api_key: String,
    pub database_url: String
}

pub fn load_config() -> bool {
    dotenv().ok();

    let Ok(api_key) = env::var("API_KEY") else {
        let key = rand::generate_alphanumeric(256);

        eprintln!("Error: API_KEY must be provided in `.env`. If needed, here is a secure random key:");
        eprintln!("{}", key);

        return false;
    };
    
    let Ok(database_url) = env::var("DATABASE_URL") else {
        eprintln!("Error: DATABASE_URL must be provided in `.env`.");
        return false;
    };

    if let Err(_) = CONFIG.set(Config { api_key, database_url }) {
        eprintln!("Error: failed to set global config");
        return false;
    };

    return true;
}