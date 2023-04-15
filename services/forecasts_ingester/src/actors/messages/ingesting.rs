use std::fmt::Display;

use crate::data_ingester::errors::IngestError;
use crate::types::windguru::{Spot, WindguruForecasts};
use actix::Message;

#[derive(Message)]
#[rtype(result = "Result<(), IngestError>")]
pub enum IngestMsg {
    WindguruForecast(WindguruIngestForecastMsg),
}

pub struct WindguruIngestForecastMsg {
    pub forecast: WindguruForecasts,
    pub spot: Spot,
}

impl Display for IngestMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IngestMsg::WindguruForecast(_) => write!(f, "WindguruIngestForecastMsg"),
            _ => unimplemented!(),
        }
    }
}
