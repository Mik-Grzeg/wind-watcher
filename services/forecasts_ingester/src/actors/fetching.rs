use crate::data_fetcher::errors::FetchError;
use crate::data_fetcher::DataFetcher;
use crate::data_ingester::errors::IngestError;

use actix::*;

// use super::messages::{fetching::FetchNewForecastMsg};

pub struct FetchingActor<IM, OM, DF>
where
    IM: Message<Result = Result<OM, FetchError>> + Send + 'static,
    OM: Message<Result = Result<(), IngestError>> + Send + 'static,
    DF: DataFetcher<InMessage = IM, OutMessage = OM>,
{
    fetcher: DF,
}

impl<IM, OM, DF> FetchingActor<IM, OM, DF>
where
    IM: Message<Result = Result<OM, FetchError>> + Send + 'static,
    OM: Message<Result = Result<(), IngestError>> + Send + 'static,
    DF: DataFetcher<InMessage = IM, OutMessage = OM>,
{
    pub fn new(fetcher: DF) -> Self {
        Self { fetcher }
    }
}

impl<IM, OM, DF> Actor for FetchingActor<IM, OM, DF>
where
    IM: Message<Result = Result<OM, FetchError>> + Send + 'static,
    OM: Message<Result = Result<(), IngestError>> + Send + 'static,
    DF: DataFetcher<InMessage = IM, OutMessage = OM> + 'static,
{
    type Context = Context<Self>;
}

impl<IM, OM, DF> Handler<IM> for FetchingActor<IM, OM, DF>
where
    IM: Message<Result = Result<OM, FetchError>> + Send + 'static,
    OM: Message<Result = Result<(), IngestError>> + Send + 'static,
    DF: DataFetcher<InMessage = IM, OutMessage = OM> + Clone + 'static,
{
    type Result = ResponseFuture<Result<OM, FetchError>>;

    fn handle(&mut self, msg: IM, _ctx: &mut Context<Self>) -> Self::Result {
        Box::pin({
            let fetcher = self.fetcher.clone();
            async move {
                let ingest_msg = fetcher.fetch_forecast(msg).await?;

                Ok(ingest_msg)
            }
        })
    }
}
