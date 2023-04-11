use crate::data_fetcher::DataFetcher;
use crate::data_ingester::DataIngester;
use crate::types::windguru::{IdModel, IdSpot, WindguruForecasts};
use sqlx::Database;
use std::sync::Arc;

use actix::*;

use super::ingesting::IngestingActor;
use super::messages::{fetching::FetchNewForecastsMsg, ingesting::IngestForecastsMsg};

pub struct FetchingActor<DF, DI>
where
    DF: DataFetcher<IngestForecastsMsg, FetchNewForecastsMsg>,
    DI: DataIngester + Unpin + 'static,
{
    fetcher: DF,
    ingesting_addr: Addr<IngestingActor<DI>>,
}

impl<DF, DI> FetchingActor<DF, DI>
where
    DF: DataFetcher<IngestForecastsMsg, FetchNewForecastsMsg>,
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
    DF: DataFetcher<IngestForecastsMsg, FetchNewForecastsMsg> + 'static,
    DI: DataIngester + Unpin + Send + Sync + 'static,
{
    type Context = Context<Self>;
}

impl<DF, DI> Handler<FetchNewForecastsMsg> for FetchingActor<DF, DI>
where
    DF: DataFetcher<IngestForecastsMsg, FetchNewForecastsMsg> + Clone + 'static,
    DI: DataIngester + Unpin + Send + Sync + Clone + 'static,
{
    type Result = ResponseFuture<Result<(), anyhow::Error>>;

    fn handle(&mut self, msg: FetchNewForecastsMsg, ctx: &mut Context<Self>) -> Self::Result {
        Box::pin({
            let fetcher = self.fetcher.clone();
            let ingester_addr = self.ingesting_addr.clone();
            async move {
                let msg_to_send = fetcher.fetch_forecast(msg).await?;

                ingester_addr.send(msg_to_send).await??;
                Ok(())
            }
        })
    }
}
