use crate::{
    actors::{
        fetching::FetchingActor,
        ingesting::IngestingActor,
        messages::{
            fetching::{Fetch, WindguruForecastFetchMsg},
            ingesting::{Forecast, WindguruIngestForecastMsg},
        },
    },
    config::{DataStorage, Settings},
    data_fetcher::{errors::FetchError, FetchingClient, ForecastDataFetcher},
    data_ingester::{errors::IngestError, DataIngester},
    types::windguru::{IdModel, IdSpot},
};
use actix::*;
use sqlx::{Pool, Postgres};
use std::sync::Arc;

use tokio::task::JoinSet;

pub struct State {
    // data_fetcher: Box<dyn ForecastDataFetcher<WindguruForecasts>>,
    // data_ingester: Box<dyn DataIngester>,
}

async fn issue_fetching_msgs<DF, DI>(
    spots: &[IdSpot],
    _models: &[IdModel],
    fetcher_addr: &Addr<FetchingActor<DF>>,
    ingester_addr: &Addr<IngestingActor<DI>>,
) where
    DF: ForecastDataFetcher + Clone + 'static,
    DI: DataIngester + Clone + 'static,
{
    let mut fetch_tasks: JoinSet<Result<Result<Box<dyn Forecast>, FetchError>, MailboxError>> =
        JoinSet::new();
    let mut ingest_tasks: JoinSet<Result<Result<(), IngestError>, MailboxError>> = JoinSet::new();

    spots.iter().for_each(|spot| {
        let msg = WindguruForecastFetchMsg { spot: *spot };

        tracing::debug!(spot = spot, "issueing fetch message {}", msg);
        fetch_tasks.spawn(fetcher_addr.send(Box::new(msg)));
    });

    while let Some(res) = fetch_tasks.join_next().await {
        let res = res.unwrap().unwrap().unwrap();
        let downcasted_res = (&*res)
            .as_any()
            .downcast_ref::<WindguruIngestForecastMsg>()
            .unwrap();

        tracing::debug!(
            spot = downcasted_res.spot.id,
            "issueing ingest message {}",
            res
        );
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
