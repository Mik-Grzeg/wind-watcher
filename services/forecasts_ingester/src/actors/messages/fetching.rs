use super::ingesting::IngestForecastsMsg;
use crate::types::windguru::{IdSpot, WindguruForecasts};
use actix::Message;

#[derive(Message, Debug)]
#[rtype(result = "Result<(), anyhow::Error>")]
pub struct FetchNewForecastsMsg {
    pub spot: IdSpot,
}
