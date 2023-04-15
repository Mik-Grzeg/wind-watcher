use std::marker::PhantomData;

use crate::data_ingester::{errors::IngestError, DataIngester};

use actix::{Actor, Context, Handler, Message, ResponseFuture};

pub struct IngestingActor<T, D>
where
    D: DataIngester<T>,
{
    repository: D,
    _marker: PhantomData<T>,
}

impl<T, D> IngestingActor<T, D>
where
    D: DataIngester<T>,
{
    pub fn new(repository: D) -> Self {
        Self {
            repository,
            _marker: PhantomData,
        }
    }
}

impl<T, D> Actor for IngestingActor<T, D>
where
    T: Send + Unpin + 'static,
    D: DataIngester<T> + Unpin + 'static,
{
    type Context = Context<Self>;
}

impl<T, D> Handler<T> for IngestingActor<T, D>
where
    T: Message<Result = Result<(), IngestError>> + Send + Unpin + 'static,
    D: DataIngester<T> + Clone + Unpin + 'static,
{
    type Result = ResponseFuture<Result<(), IngestError>>;

    fn handle(&mut self, msg: T, _ctx: &mut Context<Self>) -> Self::Result {
        Box::pin({
            let repo = self.repository.clone();
            async move { repo.ingest_forecast(msg).await }
        })
    }
}
