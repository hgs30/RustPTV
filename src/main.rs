mod config;

use std::error::Error;
use config::api_config::ApiConfig;

fn main() -> Result<(), Box<dyn Error>>  {
    let config = ApiConfig::build()?;

    println!("Config built!");
    println!("Generating URL for /v3/departures/route_type/1/stop/2043/route/1881");

    let url = config.generate_url("/v3/departures/route_type/1/stop/2043/route/1881")?;

    println!("Generated URL was:");
    println!("{}", url);

    Ok(())
}