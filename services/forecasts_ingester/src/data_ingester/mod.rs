use async_trait::async_trait;
use errors::IngestError;
use std::sync::Arc;

use crate::actors::messages::ingesting::IngestMsg;

pub mod errors;
pub mod postgres_repository;

#[async_trait]
pub trait DataIngester: Send + Sync + Unpin {
    async fn ingest_forecast(&self, data: IngestMsg) -> Result<(), IngestError>;
}

#[async_trait]
impl<DI: DataIngester> DataIngester for Arc<DI>
where
    DI: DataIngester + Send + Sync,
{
    async fn ingest_forecast(&self, data: IngestMsg) -> Result<(), IngestError> {
        self.as_ref().ingest_forecast(data).await
    }
}
