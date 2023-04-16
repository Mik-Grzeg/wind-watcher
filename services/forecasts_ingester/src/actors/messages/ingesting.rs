use std::fmt::Display;

use crate::data_ingester::errors::IngestError;
use crate::types::windguru::forecast::{Spot, WindguruForecasts};
use crate::types::windguru::station::WindguruStationData;
use actix::Message;

#[derive(Message)]
#[rtype(result = "Result<(), IngestError>")]
pub enum IngestMsg {
    WindguruForecast(WindguruForecast),
    WindguruStationReading(WindguruStationData),
}

pub struct WindguruForecast {
    pub forecast: WindguruForecasts,
    pub spot: Spot,
}

impl Display for IngestMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IngestMsg::WindguruForecast(_) => write!(f, "WindguruForecastIngestMsg"),
            IngestMsg::WindguruStationReading(_) => write!(f, "WindguruStationIngestMsg"),
            _ => unimplemented!(),
        }
    }
}
