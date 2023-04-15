use actix::Message;
use async_trait::async_trait;
use reqwest::{
    cookie::{CookieStore, Jar},
    Client, ClientBuilder, Url,
};

use std::{str::FromStr, sync::Arc, time::Duration};

use crate::{data_ingester::errors::IngestError, actors::messages::{ingesting::Forecast, fetching::Fetch}};

use self::errors::FetchError;

pub mod errors;
pub mod windguru;
mod authorization;


#[async_trait]
pub trait ForecastDataFetcher: Send + Sync + Unpin {
    async fn fetch_forecast(&self, params: Box<dyn Fetch>)
        -> Result<Box<dyn Forecast>, FetchError>;
    async fn fetch_station<IM: Send, OM: Send>(&self, params: IM)
        -> Result<OM, FetchError>;
}

#[async_trait]
impl<DF: ForecastDataFetcher> ForecastDataFetcher for Arc<DF> {
    async fn fetch_forecast(
        &self,
        params: Box<dyn Fetch>,
    ) -> Result<Box<dyn Forecast>, FetchError> {
        self.as_ref().fetch_forecast(params).await
    }

    async fn fetch_station<C: Send, D: Send>(&self, params: C) -> Result<D, FetchError> {
        self.as_ref().fetch_station(params).await
    }
}

#[derive(Debug)]
pub struct FetchingClient {
    client: Client,
    url: Url,
    jar: Arc<Jar>,
}

impl FetchingClient {
    pub fn new(url: String) -> Self {
        let jar = Arc::new(Jar::default());
        let client = ClientBuilder::new()
            .connect_timeout(Duration::from_secs(30))
            .cookie_store(true)
            .cookie_provider(Arc::clone(&jar))
            .build()
            .unwrap();
        let url = Url::from_str(&url).unwrap();

        Self { client, url, jar }
    }
}

