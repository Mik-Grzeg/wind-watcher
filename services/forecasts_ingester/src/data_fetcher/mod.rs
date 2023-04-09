use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize};
use std::sync::Arc;

pub mod windguru;

#[async_trait]
pub trait DataFetcher<T: DeserializeOwned> {
    async fn fetch_forecast(&self) -> Result<T, anyhow::Error>;
}

#[async_trait]
impl<T, DF> DataFetcher<T> for Arc<DF>
where
    T: DeserializeOwned,
    DF: DataFetcher<T> + Send + Sync,
{
    async fn fetch_forecast(&self) -> Result<T, anyhow::Error> {
        self.as_ref().fetch_forecast().await
    }
}
