use std::marker::PhantomData;

use crate::data_ingester::{errors::IngestError, DataIngester};

use actix::{Actor, Context, Handler, Message, ResponseFuture};

use super::messages::ingesting::Forecast;

pub struct IngestingActor<D: DataIngester> {
    repository: D,
}

impl<D: DataIngester> IngestingActor<D> {
    pub fn new(repository: D) -> Self {
        Self {
            repository,
        }
    }
}

impl<D> Actor for IngestingActor<D>
where
    D: DataIngester + Unpin + 'static,
{
    type Context = Context<Self>;
}

impl<D> Handler<Box<dyn Forecast>> for IngestingActor<D>
where
    D: DataIngester + Clone + Unpin + 'static,
{
    type Result = ResponseFuture<Result<(), IngestError>>;

    fn handle(&mut self, msg: Box<dyn Forecast>, _ctx: &mut Context<Self>) -> Self::Result {
        Box::pin({
            let repo = self.repository.clone();
            async move { repo.ingest_forecast(msg).await }
        })
    }
}
