use async_trait::async_trait;
use std::sync::Arc;



pub mod postgres_repository;

#[async_trait]
trait DataIngester<T> {
    async fn ingest_forecast(&self, data: T) -> Result<(), anyhow::Error>;
}

#[async_trait]
impl<T, DI> DataIngester<T> for Arc<DI>
where
    T: Send + 'static,
    DI: DataIngester<T> + Send + Sync,
{
    async fn ingest_forecast(&self, data: T) -> Result<(), anyhow::Error> {
        self.as_ref().ingest_forecast(data).await
    }
}
