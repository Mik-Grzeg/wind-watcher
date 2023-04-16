use std::str::FromStr;

use crate::data_fetcher::FetchError;

use crate::data_fetcher::DataFetcher;
use crate::data_fetcher::FetchMsg;
use crate::data_fetcher::IngestMsg;
use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use reqwest::cookie::CookieStore;
use reqwest::{cookie::Jar, Client, ClientBuilder, Url};

use super::authorization::Authorizer;
use super::windguru;
use tracing::instrument;

#[derive(Debug)]
pub struct FetchingClient {
    pub client: Client,
    pub url: Url,
    pub jar: Arc<Jar>,
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

#[async_trait]
impl DataFetcher for FetchingClient {
    #[instrument(skip(self, params))]
    async fn fetch(&self, params: FetchMsg) -> Result<IngestMsg, FetchError> {
        self.authorize().await?;

        match params {
            FetchMsg::WindguruForecast(params) => {
                windguru::forecasts::get_forecast(self, params).await
            }
            FetchMsg::WindguruStation(params) => {
                windguru::stations::get_station_data(self, params).await
            }
            _ => unimplemented!(),
        }
    }
}

#[async_trait]
impl Authorizer for FetchingClient {
    async fn authorize(&self) -> Result<(), FetchError> {
        // Get authorization cookies
        let response = self.client.get(self.url.clone()).send().await?;

        let cookies = self.jar.cookies(&self.url);
        let cookies = cookies.ok_or(FetchError::MissingCookies)?;

        tracing::debug!(
            cookies = cookies.to_str()?,
            status = response.status().as_u16(),
            "Fetching auth cookies"
        );

        Ok(())
    }
}
