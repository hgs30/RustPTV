mod ptv_api;
mod models;

use std::error::Error;
use chrono::Duration;
use reqwest::blocking::Client;
use ptv_api::ptv_api_client::PtvApiClient;
use timer::Timer;
use crate::models::departure::{Departure, Departures};

fn main() -> Result<(), Box<dyn Error>>  {
    let client = PtvApiClient::build()?;

    let request = "/v3/departures/route_type/1/stop/2043/route/1881?direction_id=28&max_results=3";

    println!("Client built!");
    println!("Generating URL for {request}");

    let url = client.generate_url(request)?;

    println!("Generated URL was:");
    println!("{}", &url);

    let client = Client::new();

    let departures = get_departures(&client, &url)?;

    for departure in departures {
        println!("{:?}", departure)
    }

    let timer = Timer::new();

    let _guard = timer.schedule_repeating(Duration::new(60, 0).unwrap(), move || {
        let departures = get_departures(&client, &url).unwrap();

        for departure in departures {
            println!("{:?}", departure)
        }
    });

    loop {}
}

fn get_departures(client: &Client, url: &str) -> Result<Vec<Departure>, Box<dyn Error>> {
    let response = client.get(url)
        .send()?;

    if !response.status().is_success() {
        return Err(Box::from("Request was not successful"))
    }

    let departures: Departures = response.json()?;

    Ok(departures.departures)
}