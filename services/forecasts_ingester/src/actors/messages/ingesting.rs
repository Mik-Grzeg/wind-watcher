use std::any::Any;
use std::fmt::Display;

use crate::data_ingester::errors::IngestError;
use crate::types::windguru::{Spot, WindguruForecasts};
use actix::Message;

#[derive(Message)]
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

impl Forecast for WindguruIngestForecastMsg { 
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn boxed_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}

pub trait Forecast: Message<Result = Result<(), IngestError>> + Display + Send + Unpin + 'static {
    fn as_any(&self) -> &dyn Any;
    fn boxed_any(self) -> Box<dyn Any>;
}

impl Message for Box<dyn Forecast> {
    type Result = Result<(), IngestError>;
}
