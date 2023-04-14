use actix::Message;
use async_trait::async_trait;
use reqwest::{cookie::{Jar, CookieStore}, Client, ClientBuilder, Url};
use serde::de::DeserializeOwned;
use std::{str::FromStr, sync::Arc, time::Duration};

use crate::actors::messages::{
    fetching::FetchNewForecastsMsg, ingesting::WindguruIngestForecastMsg,
};

use self::errors::FetchingError;

mod errors;
pub mod windguru;

#[async_trait]
pub trait Authorizer {
    async fn authorize(&self) -> Result<(), FetchingError>;
}

#[async_trait]
pub trait DataFetcher: Send + Sync + Unpin {
    type OutMessage;
    type InMessage;

    async fn fetch_forecast(
        &self,
        params: Self::InMessage,
    ) -> Result<Self::OutMessage, FetchingError>;
}

#[async_trait]
impl<OM, IM, DF: DataFetcher<InMessage = IM, OutMessage = OM>> DataFetcher for Arc<DF>
where
    OM: Message + Send,
    IM: Message + Send,
{
    type InMessage = IM;
    type OutMessage = OM;

    async fn fetch_forecast(
        &self,
        params: Self::InMessage,
    ) -> Result<Self::OutMessage, FetchingError> {
        self.as_ref().fetch_forecast(params).await
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

#[async_trait]
impl Authorizer for FetchingClient {
    async fn authorize(&self) -> Result<(), FetchingError> {
        // Get authorization cookies
        let response = self.client.get(self.url.clone()).send().await?;

        let cookies = self.jar.cookies(&self.url);
        let cookies = cookies.ok_or(FetchingError::MissingCookies)?;

        tracing::debug!(
            cookies = cookies.to_str()?,
            status = response.status().as_u16(),
            "Fetching auth cookies"
        );

        Ok(())
    }
}
