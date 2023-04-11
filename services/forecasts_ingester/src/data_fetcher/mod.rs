use async_trait::async_trait;
use serde::de::DeserializeOwned;
use std::sync::Arc;

pub mod windguru;

#[async_trait]
pub trait DataFetcher<T, P>: Send + Sync + Unpin {
    async fn fetch_forecast(&self, params: P) -> Result<T, anyhow::Error>;
}

#[async_trait]
impl<T, DF, P> DataFetcher<T, P> for Arc<DF>
where
    P: Send + Sync + 'static,
    DF: DataFetcher<T, P> + Send + Sync,
{
    async fn fetch_forecast(&self, params: P) -> Result<T, anyhow::Error> {
        self.as_ref().fetch_forecast(params).await
    }
}
