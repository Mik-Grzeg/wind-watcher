use crate::data_fetcher::errors::FetchError;
use crate::data_fetcher::ForecastDataFetcher;

use actix::*;

use super::messages::fetching::FetchMsg;
use super::messages::ingesting::IngestMsg;

// use super::messages::{fetching::FetchNewForecastMsg};

pub struct FetchingActor<DF>
where
    // IM: Message<Result = Result<OM, FetchError>> + Send + 'static,
    // OM: Message<Result = Result<(), IngestError>> + Send + 'static,
    DF: ForecastDataFetcher,
{
    fetcher: DF,
}

impl<DF> FetchingActor<DF>
where
    // IM: Message<Result = Result<OM, FetchError>> + Send + 'static,
    // OM: Message<Result = Result<(), IngestError>> + Send + 'static,
    DF: ForecastDataFetcher,
{
    pub fn new(fetcher: DF) -> Self {
        Self { fetcher }
    }
}

impl<DF> Actor for FetchingActor<DF>
where
    // IM: Message<Result = Result<OM, FetchError>> + Send + 'static,
    // OM: Message<Result = Result<(), IngestError>> + Send + 'static,
    DF: ForecastDataFetcher + 'static,
{
    type Context = Context<Self>;
}

impl<DF> Handler<FetchMsg> for FetchingActor<DF>
where
    // IM: Message<Result = Result<OM, FetchError>> + Send + 'static,
    // OM: Message<Result = Result<(), IngestError>> + Send + 'static,
    DF: ForecastDataFetcher + Clone + 'static,
{
    type Result = ResponseFuture<Result<IngestMsg, FetchError>>;

    fn handle(&mut self, msg: FetchMsg, _ctx: &mut Context<Self>) -> Self::Result {
        Box::pin({
            let fetcher = self.fetcher.clone();
            async move {
                let ingest_msg = fetcher.fetch_forecast(msg).await?;

                Ok(ingest_msg)
            }
        })
    }
}
