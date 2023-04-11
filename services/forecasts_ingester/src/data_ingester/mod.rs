use async_trait::async_trait;
use std::sync::Arc;

use crate::{actors::messages::ingesting::IngestForecastsMsg, types::windguru::WindguruForecasts};

pub mod postgres_repository;

#[async_trait]
pub trait DataIngester: Send + Sync + Unpin {
    async fn ingest_forecast(&self, data: IngestForecastsMsg) -> Result<(), anyhow::Error>;
}

#[async_trait]
impl<DI> DataIngester for Arc<DI>
where
    DI: DataIngester + Send + Sync,
{
    async fn ingest_forecast(&self, data: IngestForecastsMsg) -> Result<(), anyhow::Error> {
        self.as_ref().ingest_forecast(data).await
    }
}
