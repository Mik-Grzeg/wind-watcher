use crate::data_fetcher::DataFetcher;
use crate::data_ingester::DataIngester;
use crate::types::windguru::{IdModel, IdSpot, WindguruForecasts};
use sqlx::Database;
use std::sync::Arc;

use actix::*;

use super::ingesting::IngestingActor;
use super::messages::{fetching::FetchNewForecastsMsg, ingesting::WindguruIngestForecastMsg};

pub struct FetchingActor<DF, DI>
where
    DF: DataFetcher,
    DI: DataIngester + Unpin + 'static,
{
    fetcher: DF,
    ingesting_addr: Addr<IngestingActor<DI>>,
}

impl<DF, DI> FetchingActor<DF, DI>
where
    DF: DataFetcher,
    DI: DataIngester + Unpin,
{
    pub fn new(fetcher: DF, ingesting_addr: Addr<IngestingActor<DI>>) -> Self {
        Self {
            fetcher,
            ingesting_addr,
        }
    }
}

impl<DF, DI> Actor for FetchingActor<DF, DI>
where
    DF: DataFetcher + 'static,
    DI: DataIngester + Unpin + Send + Sync + 'static,
{
    type Context = Context<Self>;
}

impl<IM, OM, DF, DI> Handler<IM> for FetchingActor<DF, DI>
where
    IM: Message + Send,
    OM: Message + Send,
    DF: DataFetcher<OutMessage = OM, InMessage = IM> + Clone + 'static,
    DI: DataIngester + Unpin + Send + Sync + Clone + 'static,
{
    type Result = ResponseFuture<Result<(), anyhow::Error>>;

    fn handle(&mut self, msg: IM, ctx: &mut Context<Self>) -> Self::Result {
        Box::pin({
            let fetcher = self.fetcher.clone();
            let ingester_addr = self.ingesting_addr.clone();
            async move {
                let msg_to_send: OM = fetcher.fetch_forecast(msg).await?;

                ingester_addr.send(msg_to_send).await??;
                Ok(())
            }
        })
    }
}
