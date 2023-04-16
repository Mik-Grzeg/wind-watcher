use crate::data_fetcher::errors::FetchError;
use async_trait::async_trait;

#[async_trait]
pub trait Authorizer {
    async fn authorize(&self) -> Result<(), FetchError>;
}
