mod config;

use config::api_config::ApiConfig;

fn main() {
    match ApiConfig::build() {
        Ok(config) => {
            println!("Dev ID: {}", config.dev_id);
            println!("API Key: {}", config.key);
        }
        Err(e) => eprintln!("Failed to build ApiConfig: {}", e),
    }
}