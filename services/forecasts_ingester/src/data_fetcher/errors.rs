use reqwest::Url;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FetchingError {
    #[error("missing cookie header")]
    MissingCookies,
    #[error("unable to parse cookies to str")]
    InvalidCookies(#[from] reqwest::header::ToStrError),
    #[error("sending request failed err={0}")]
    ErrorFetchingRequest(#[from] reqwest::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error)
}

