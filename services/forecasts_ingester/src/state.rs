use crate::{
    actors::{
        fetching::FetchingActor,
        ingesting::IngestingActor,
        messages::{
            fetching::{FetchMsg, WindguruForecastFetchMsg},
            ingesting::IngestMsg,
        },
    },
    config::{DataStorage, Settings},
    data_fetcher::{client::FetchingClient, errors::FetchError, DataFetcher},
    data_ingester::{errors::IngestError, DataIngester},
    types::windguru::{
        forecast::{IdModel, IdSpot},
        station::WindguruStationFetchParams,
    },
};
use actix::*;
use chrono::{DateTime, Duration, Utc};
use sqlx::{Pool, Postgres};
use std::sync::Arc;

use tokio::task::JoinSet;

pub struct State {
    // data_fetcher: Box<dyn DataFetcher<WindguruForecasts>>,
    // data_ingester: Box<dyn DataIngester>,
}

fn get_yesterday_date_bounds() -> (DateTime<Utc>, DateTime<Utc>) {
    let now = Utc::now();
    let yesterday_start = DateTime::from_utc(now.date_naive().and_hms_opt(0, 0, 0).unwrap(), Utc);
    let yesterday_end = yesterday_start + Duration::minutes(23 * 60 + 59);

    (yesterday_start, yesterday_end)
}

async fn issue_fetching_msgs<DF, DI>(
    spots: &[IdSpot],
    _models: &[IdModel],
    fetcher_addr: &Addr<FetchingActor<DF>>,
    ingester_addr: &Addr<IngestingActor<DI>>,
) where
    DF: DataFetcher + Clone + 'static,
    DI: DataIngester + Clone + 'static,
{
    let mut fetch_tasks: JoinSet<Result<Result<IngestMsg, FetchError>, MailboxError>> =
        JoinSet::new();
    let mut ingest_tasks: JoinSet<Result<Result<(), IngestError>, MailboxError>> = JoinSet::new();
    let (start, end) = get_yesterday_date_bounds();

    spots.iter().for_each(|spot| {
        let forecast_msg = FetchMsg::WindguruForecast(WindguruForecastFetchMsg { spot: *spot });
        let station_msg = FetchMsg::WindguruStation(WindguruStationFetchParams {
            id_station: 2764,
            from: start,
            to: end,
            avg_minutes: 5,
            ..Default::default()
        });

        tracing::debug!(spot = spot, "issueing forecast fetch message {}", forecast_msg);
        fetch_tasks.spawn(fetcher_addr.send(forecast_msg));

        tracing::debug!(
            station = 2764,
            "issueing station fetch message {}",
            station_msg
        );
        fetch_tasks.spawn(fetcher_addr.send(station_msg));
    });

    while let Some(res) = fetch_tasks.join_next().await {
        let res = res.unwrap().unwrap();
        match res {
            Ok(msg) => {
                tracing::debug!("issueing ingest message {}", msg);
                ingest_tasks.spawn(ingester_addr.send(msg));
            },
            Err(err) => {
                tracing::error!("error after fetching message {}", err);
            }
        }
    }

    while let Some(res) = ingest_tasks.join_next().await {
        let _ = res.unwrap().unwrap().unwrap();
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
