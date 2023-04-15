use crate::{
    actors::{
        fetching::FetchingActor,
        ingesting::IngestingActor,
        messages::{fetching::WindguruForecastFetchMsg, ingesting::WindguruIngestForecastMsg},
    },
    config::{DataStorage, Settings},
    data_fetcher::{errors::FetchError, DataFetcher, FetchingClient},
    data_ingester::{errors::IngestError, DataIngester},
    types::windguru::{IdModel, IdSpot},
};
use actix::*;
use sqlx::{Pool, Postgres};
use std::sync::Arc;

use tokio::task::JoinSet;

pub struct State {
    // data_fetcher: Box<dyn DataFetcher<WindguruForecasts>>,
    // data_ingester: Box<dyn DataIngester>,
}

async fn issue_fetching_msgs<DF, DI>(
    spots: &[IdSpot],
    _models: &[IdModel],
    fetcher_addr: &Addr<FetchingActor<WindguruForecastFetchMsg, WindguruIngestForecastMsg, DF>>,
    ingester_addr: &Addr<IngestingActor<WindguruIngestForecastMsg, DI>>,
) where
    DF: DataFetcher<InMessage = WindguruForecastFetchMsg, OutMessage = WindguruIngestForecastMsg>
        + Clone
        + 'static,
    DI: DataIngester<WindguruIngestForecastMsg> + Clone + 'static,
{
    let mut fetch_tasks: JoinSet<
        Result<Result<WindguruIngestForecastMsg, FetchError>, MailboxError>,
    > = JoinSet::new();
    let mut ingest_tasks: JoinSet<Result<Result<(), IngestError>, MailboxError>> = JoinSet::new();

    spots.iter().for_each(|spot| {
        let msg = WindguruForecastFetchMsg { spot: *spot };

        tracing::debug!(spot = spot, "issueing fetch message {}", msg);
        fetch_tasks.spawn(fetcher_addr.send(msg));
    });

    while let Some(res) = fetch_tasks.join_next().await {
        let res = res.unwrap().unwrap().unwrap();

        tracing::debug!(spot = res.spot.id, "issueing ingest message {}", res);
        ingest_tasks.spawn(ingester_addr.send(res));
    }

    while let Some(res) = ingest_tasks.join_next().await {
        res.unwrap().unwrap().unwrap();
        tracing::debug!("successully ingested data");
    }
}

impl State {
    pub async fn start(settings: Settings) {
        let data_fetcher = Arc::new(FetchingClient::from(&settings));

        let data_ingester = match settings.storage {
            DataStorage::Postgresql(config) => Pool::<Postgres>::connect(&config.connection_url)
                .await
                .unwrap(),
            _ => unimplemented!(),
        };

        let fetcher_addr = FetchingActor::new(data_fetcher).start();
        let ingester_addr = IngestingActor::new(data_ingester).start();

        issue_fetching_msgs(
            &settings.windguru.spots,
            &settings.windguru.models,
            &fetcher_addr,
            &ingester_addr,
        )
        .await;
    }
}
