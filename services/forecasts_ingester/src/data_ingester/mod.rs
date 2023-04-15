use async_trait::async_trait;
use errors::IngestError;
use std::sync::Arc;

pub mod errors;
pub mod postgres_repository;

#[async_trait]
pub trait DataIngester<T>: Send + Sync + Unpin {
    async fn ingest_forecast(&self, data: T) -> Result<(), IngestError>;
}

#[async_trait]
impl<T, DI: DataIngester<T>> DataIngester<T> for Arc<DI>
where
    T: Send + 'static,
    DI: DataIngester<T> + Send + Sync,
{
    async fn ingest_forecast(&self, data: T) -> Result<(), IngestError> {
        self.as_ref().ingest_forecast(data).await
    }
}
