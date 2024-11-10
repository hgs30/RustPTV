use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Departures {
    pub departures: Vec<Departure>,
}

#[derive(Deserialize, Debug)]
pub struct Departure {
    pub stop_id: i32,
    pub route_id: i32,
    pub scheduled_departure_utc: String,
}