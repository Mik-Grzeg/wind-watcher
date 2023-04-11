use crate::types::windguru::{Spot, WindguruForecasts};
use actix::Message;

#[derive(Message)]
#[rtype(result = "Result<(), anyhow::Error>")]
pub struct IngestForecastsMsg {
    pub forecast: WindguruForecasts,
    pub spot: Spot,
}