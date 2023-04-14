use crate::data_ingester::DataIngester;
use crate::types::windguru::WindguruForecasts;
use actix::{Actor, Context, Handler, ResponseFuture};
use sqlx::{query::Query, Database, Pool, Postgres, QueryBuilder};

use super::messages::ingesting::WindguruIngestForecastMsg;

pub struct IngestingActor<D>
where
    D: DataIngester,
{
    repository: D,
}

impl<D> IngestingActor<D>
where
    D: DataIngester,
{
    pub fn new(repository: D) -> Self {
        Self { repository }
    }
}

impl<D> Actor for IngestingActor<D>
where
    D: DataIngester + Unpin + 'static,
{
    type Context = Context<Self>;
}

impl<D> Handler<WindguruIngestForecastMsg> for IngestingActor<D>
where
    D: DataIngester + Clone + Unpin + 'static,
{
    type Result = ResponseFuture<Result<(), anyhow::Error>>;

    fn handle(&mut self, msg: WindguruIngestForecastMsg, ctx: &mut Context<Self>) -> Self::Result {
        Box::pin({
            let repo = self.repository.clone();
            async move { repo.ingest_forecast(msg).await }
        })
    }
}
