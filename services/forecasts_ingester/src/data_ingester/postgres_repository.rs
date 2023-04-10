use super::DataIngester;
use async_trait::async_trait;
use sqlx::PgPool;

// struct PostgresRepository {
//     pub
// }

#[async_trait]
impl<T: Send + 'static> DataIngester<T> for PgPool {
    async fn ingest_forecast(&self, _data: T) -> Result<(), anyhow::Error> {
        unimplemented!()
    }
}
