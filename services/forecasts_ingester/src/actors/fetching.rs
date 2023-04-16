use crate::data_fetcher::errors::FetchError;
use crate::data_fetcher::DataFetcher;

use actix::*;

use super::messages::fetching::FetchMsg;
use super::messages::ingesting::IngestMsg;

pub struct FetchingActor<DF: DataFetcher> {
    fetcher: DF,
}

impl<DF: DataFetcher> FetchingActor<DF> {
    pub fn new(fetcher: DF) -> Self {
        Self { fetcher }
    }
}

impl<DF> Actor for FetchingActor<DF>
where
    // IM: Message<Result = Result<OM, FetchError>> + Send + 'static,
    // OM: Message<Result = Result<(), IngestError>> + Send + 'static,
    DF: DataFetcher + 'static,
{
    type Context = Context<Self>;
}

impl<DF> Handler<FetchMsg> for FetchingActor<DF>
where
    DF: DataFetcher + Clone + 'static,
{
    type Result = ResponseFuture<Result<IngestMsg, FetchError>>;

    fn handle(&mut self, msg: FetchMsg, _ctx: &mut Context<Self>) -> Self::Result {
        Box::pin({
            let fetcher = self.fetcher.clone();
            async move {
                let ingest_msg = fetcher.fetch(msg).await?;

                Ok(ingest_msg)
            }
        })
    }
}
