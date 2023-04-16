use std::fmt::Display;

use super::ingesting::IngestMsg;
use crate::data_fetcher::errors::FetchError;
use crate::types::windguru::forecast::IdSpot;
use crate::types::windguru::station::WindguruStationFetchParams;
use actix::Message;

#[derive(Message)]
#[rtype(result = "Result<IngestMsg, FetchError>")]
pub enum FetchMsg {
    WindguruForecast(WindguruForecastFetchMsg),
    WindguruStation(WindguruStationFetchParams),
}

pub struct WindguruForecastFetchMsg {
    pub spot: IdSpot,
}

impl Display for FetchMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FetchMsg::WindguruForecast(_) => write!(f, "WindguruForecastFetchMsg"),
            FetchMsg::WindguruStation(_) => write!(f, "WindguruStationFetchMsg"),
            _ => unimplemented!(),
        }
    }
}
