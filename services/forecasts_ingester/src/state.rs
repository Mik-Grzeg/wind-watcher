use crate::{
    actors::{
        fetching::FetchingActor,
        ingesting::IngestingActor,
        messages::{fetching::FetchNewForecastsMsg, ingesting::IngestForecastsMsg},
    },
    config::{DataStorage, Settings},
    data_fetcher::{windguru::WindguruSpotClient, DataFetcher},
    data_ingester::DataIngester,
    types::windguru::{IdModel, IdSpot, WindguruForecasts},
};
use actix::*;
use sqlx::{Database, Pool, Postgres};
use std::sync::Arc;
use tokio::time::{self, Duration};

pub struct State {
    // data_fetcher: Box<dyn DataFetcher<WindguruForecasts>>,
    // data_ingester: Box<dyn DataIngester>,
}

async fn issue_fetching_msgs<DF, DI>(
    spots: &Vec<IdSpot>,
    _models: &Vec<IdModel>,
    fetcher_addr: &Addr<FetchingActor<DF, DI>>,
) where
    DF: DataFetcher<IngestForecastsMsg, FetchNewForecastsMsg> + Clone + 'static,
    DI: DataIngester + Clone + 'static,
{
    for spot in spots {
        fetcher_addr
            .send(FetchNewForecastsMsg { spot: *spot })
            .await
            .unwrap()
            .unwrap();
    }
}

impl State {
    pub async fn start(settings: Settings) {
        let data_fetcher = Arc::new(WindguruSpotClient::from(&settings));

        let data_ingester = match settings.storage {
            DataStorage::Postgresql(config) => Pool::<Postgres>::connect(&config.connection_url)
                .await
                .unwrap(),
            _ => unimplemented!(),
        };

        let ingester_addr = IngestingActor::new(data_ingester).start();
        let fetcher_addr = FetchingActor::new(data_fetcher, ingester_addr).start();

        issue_fetching_msgs(
            &settings.windguru.spots,
            &settings.windguru.models,
            &fetcher_addr,
        )
        .await;
    }
}
