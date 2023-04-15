use actix::Message;
use async_trait::async_trait;
use reqwest::{
    cookie::{CookieStore, Jar},
    Client, ClientBuilder, Url,
};

use std::{str::FromStr, sync::Arc, time::Duration};

use crate::data_ingester::errors::IngestError;

use self::errors::FetchError;

pub mod errors;
pub mod windguru;
mod authorization;


#[async_trait]
pub trait DataFetcher: Send + Sync + Unpin {
    type OutMessage;
    type InMessage;

    async fn fetch_forecast(&self, params: Self::InMessage)
        -> Result<Self::OutMessage, FetchError>;
}

#[async_trait]
impl<OM, IM, DF: DataFetcher<InMessage = IM, OutMessage = OM>> DataFetcher for Arc<DF>
where
    IM: Message<Result = Result<OM, FetchError>> + Send + 'static,
    OM: Message<Result = Result<(), IngestError>> + Send + 'static,
{
    type InMessage = IM;
    type OutMessage = OM;

    async fn fetch_forecast(
        &self,
        params: Self::InMessage,
    ) -> Result<Self::OutMessage, FetchError> {
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

