use std::fmt::Display;

use crate::data_ingester::errors::IngestError;
use crate::types::windguru::{Spot, WindguruForecasts};
use actix::Message;

#[derive(Message, Debug)]
#[rtype(result = "Result<(), IngestError>")]
pub struct WindguruIngestForecastMsg {
    pub forecast: WindguruForecasts,
    pub spot: Spot,
}

impl Display for WindguruIngestForecastMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WindguruIngestForecastMsg")
    }
}
