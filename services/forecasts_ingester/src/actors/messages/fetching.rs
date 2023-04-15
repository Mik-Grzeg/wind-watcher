use std::any::Any;
use std::fmt::{Display, Debug};

use super::ingesting::{WindguruIngestForecastMsg, Forecast};
use crate::data_fetcher::errors::FetchError;
use crate::types::windguru::IdSpot;
use actix::Message;

#[derive(Message)]
#[rtype(result = "Result<Box<dyn Forecast>, FetchError>")]
pub struct WindguruForecastFetchMsg {
    pub spot: IdSpot,
}

impl Display for WindguruForecastFetchMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WindguruForecastFetchMsg")
    }
}

impl Fetch for WindguruForecastFetchMsg {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Message for dyn Fetch {
    type Result = Result<Box<dyn Forecast>, FetchError>;
}

impl Message for Box<dyn Fetch> {
    type Result = Result<Box<dyn Forecast>, FetchError>;
}

pub trait Fetch: Send {
    fn as_any(&self) -> &dyn Any;
}

