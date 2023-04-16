use async_trait::async_trait;

use std::sync::Arc;

use crate::actors::messages::{fetching::FetchMsg, ingesting::IngestMsg};

use self::errors::FetchError;

mod authorization;
pub mod client;
pub mod errors;
pub mod windguru;

#[async_trait]
pub trait DataFetcher: Send + Sync + Unpin {
    async fn fetch(&self, params: FetchMsg) -> Result<IngestMsg, FetchError>;
}

#[async_trait]
impl<DF: DataFetcher> DataFetcher for Arc<DF> {
    async fn fetch(&self, params: FetchMsg) -> Result<IngestMsg, FetchError> {
        self.as_ref().fetch(params).await
    }
}
