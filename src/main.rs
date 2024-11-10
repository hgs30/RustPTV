mod ptv_api;
mod models;

use std::error::Error;
use ptv_api::ptv_api_client::PtvApiClient;
use crate::models::departure::{Departure, Departures};

fn main() -> Result<(), Box<dyn Error>>  {
    let client = PtvApiClient::build()?;

    let request = "/v3/departures/route_type/1/stop/2043/route/1881?direction_id=28&date_utc=2024-11-10T01:58:17+0000";

    println!("Client built!");
    println!("Generating URL for {request}");

    let url = client.generate_url(request)?;

    println!("Generated URL was:");
    println!("{}", url);

    let client = reqwest::blocking::Client::new();

    let response = client.get(url)
        .send()?;

    if response.status().is_success() {
        let departures: Departures = response.json()?;

        for departure in departures.departures {
            println!("{:?}", departure)
        }
    }

    Ok(())
}