use std::fmt::Display;

use super::ingesting::WindguruIngestForecastMsg;
use crate::data_fetcher::errors::FetchError;
use crate::types::windguru::IdSpot;
use actix::Message;

#[derive(Message, Debug)]
#[rtype(result = "Result<WindguruIngestForecastMsg, FetchError>")]
pub struct WindguruForecastFetchMsg {
    pub spot: IdSpot,
}

impl Display for WindguruForecastFetchMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WindguruForecastFetchMsg")
    }
}
