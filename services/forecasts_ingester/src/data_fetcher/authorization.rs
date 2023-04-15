use super::FetchingClient;
use crate::data_fetcher::errors::FetchError;
use async_trait::async_trait;
use reqwest::cookie::CookieStore;

#[async_trait]
pub trait Authorizer {
    async fn authorize(&self) -> Result<(), FetchError>;
    // fn is_authorized(&self, required_cookies: &[&str]) -> bool;
}

#[async_trait]
impl Authorizer for FetchingClient {
    async fn authorize(&self) -> Result<(), FetchError> {
        // Get authorization cookies
        let response = self.client.get(self.url.clone()).send().await?;

        let cookies = self.jar.cookies(&self.url);
        let cookies = cookies.ok_or(FetchError::MissingCookies)?;

        tracing::debug!(
            cookies = cookies.to_str()?,
            status = response.status().as_u16(),
            "Fetching auth cookies"
        );

        Ok(())
    }
}
