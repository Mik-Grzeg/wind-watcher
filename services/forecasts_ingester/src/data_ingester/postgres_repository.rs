use crate::actors::messages::{ingesting::{WindguruIngestForecastMsg, Forecast}, fetching::Fetch};

use std::any::Any;
use anyhow::anyhow;
use super::errors::IngestError;
use super::DataIngester;
use async_trait::async_trait;
use sqlx::{postgres::PgDatabaseError, PgPool, QueryBuilder};

#[async_trait]
impl DataIngester for PgPool {
    async fn ingest_forecast<T: Fetch>(&self, data: T) -> Result<(), IngestError> {
    // async fn ingest_forecast(&self, data: Box<dyn Forecast>) -> Result<(), IngestError> {
        // let WindguruIngestForecastMsg { forecast, spot } = (&*data).as_any().downcast_ref::<WindguruIngestForecastMsg>().unwrap();
        let Some(boxed_data) = data.as_any().downcast_ref::<WindguruIngestForecastMsg>() else { Err(anyhow!("unable to downcast Forecast msg"))? };

        let mut query_builder = QueryBuilder::new(
            r#"INSERT INTO forecasts(
                id_spot,
                id_model,
                forecast_from,
                forecast_for,
                wave,
                gust,
                wind_speed,
                wind_direction,
                temperature,
                relative_humidity,
                precipitation,
                cloud_cover_high,
                cloud_cover_mid,
                cloud_cover_low
            ) "#,
        );
        query_builder.push_values(Vec::clone(&boxed_data.forecast.forecasts), |mut b, fcst| {
            b.push_bind(boxed_data.forecast.id_spot)
                .push_bind(boxed_data.forecast.wgmodel.id_model)
                .push_bind(fcst.forecast_from)
                .push_bind(fcst.forecast_for)
                .push_bind(boxed_data.forecast.wgmodel.wave)
                .push_bind(fcst.gust)
                .push_bind(fcst.wind_speed)
                .push_bind(fcst.wind_direction)
                .push_bind(fcst.temperature)
                .push_bind(fcst.relative_humidity)
                .push_bind(fcst.precipitation)
                .push_bind(fcst.cloud_cover_high)
                .push_bind(fcst.cloud_cover_mid)
                .push_bind(fcst.cloud_cover_low);
        });

        sqlx::query("INSERT INTO spots (id, name, country, models) VALUES ($1, $2, $3, $4) ON CONFLICT (id) DO NOTHING")
            .bind(&boxed_data.spot.id)
            .bind(&boxed_data.spot.name)
            .bind(&boxed_data.spot.country)
            .bind(&boxed_data.spot.models)
            .execute(self).await?;

        sqlx::query(
            "INSERT INTO models (id, identifier, name) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
        )
        .bind(&boxed_data.forecast.wgmodel.id_model)
        .bind(&boxed_data.forecast.wgmodel.model)
        .bind(&boxed_data.forecast.wgmodel.model_name)
        .execute(self)
        .await?;

        match query_builder.build().execute(self).await {
            Ok(rows_affected) => {
                tracing::debug!(
                    rows_affected = rows_affected.rows_affected(),
                    "sucessfuly inserted data to postgres storage"
                );
                Ok(())
            }
            Err(error) => handle_pg_errors(error),
        }
    }
}

fn handle_pg_errors(error: sqlx::error::Error) -> Result<(), IngestError> {
    match error {
        sqlx::error::Error::Database(db_err) => {
            match db_err.code() {
                // 23505 - key is duplicated
                // in this case it is acceptable behaviour
                Some(std::borrow::Cow::Borrowed("23505")) => {
                    let pg_err: Box<PgDatabaseError> = db_err.downcast();

                    tracing::debug!(
                        table = pg_err.table(),
                        constraint = pg_err.constraint(),
                        "data already present details={:?}",
                        pg_err.detail()
                    );
                    Ok(())
                }
                _ => Err(db_err.into()),
            }
        }
        _ => Err(error.into()),
    }
}

// #[async_trait]
// impl DataIngester<WindguruIngestForecastMsg> for PgPool {
//     async fn ingest_forecast(&self, data: WindguruIngestForecastMsg) -> Result<(), IngestError> {
//         let WindguruIngestForecastMsg { forecast, spot } = data;

//         let mut query_builder = QueryBuilder::new(
//             r#"INSERT INTO forecasts(
//                 id_spot,
//                 id_model,
//                 forecast_from,
//                 forecast_for,
//                 wave,
//                 gust,
//                 wind_speed,
//                 wind_direction,
//                 temperature,
//                 relative_humidity,
//                 precipitation,
//                 cloud_cover_high,
//                 cloud_cover_mid,
//                 cloud_cover_low
//             ) "#,
//         );
//         query_builder.push_values(forecast.forecasts, |mut b, fcst| {
//             b.push_bind(forecast.id_spot)
//                 .push_bind(forecast.wgmodel.id_model)
//                 .push_bind(fcst.forecast_from)
//                 .push_bind(fcst.forecast_for)
//                 .push_bind(forecast.wgmodel.wave)
//                 .push_bind(fcst.gust)
//                 .push_bind(fcst.wind_speed)
//                 .push_bind(fcst.wind_direction)
//                 .push_bind(fcst.temperature)
//                 .push_bind(fcst.relative_humidity)
//                 .push_bind(fcst.precipitation)
//                 .push_bind(fcst.cloud_cover_high)
//                 .push_bind(fcst.cloud_cover_mid)
//                 .push_bind(fcst.cloud_cover_low);
//         });

//         sqlx::query("INSERT INTO spots (id, name, country, models) VALUES ($1, $2, $3, $4) ON CONFLICT (id) DO NOTHING")
//             .bind(spot.id)
//             .bind(spot.name)
//             .bind(spot.country)
//             .bind(spot.models)
//             .execute(self).await?;

//         sqlx::query(
//             "INSERT INTO models (id, identifier, name) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
//         )
//         .bind(forecast.wgmodel.id_model)
//         .bind(forecast.wgmodel.model)
//         .bind(forecast.wgmodel.model_name)
//         .execute(self)
//         .await?;

//         match query_builder.build().execute(self).await {
//             Ok(rows_affected) => {
//                 tracing::debug!(
//                     rows_affected = rows_affected.rows_affected(),
//                     "sucessfuly inserted data to postgres storage"
//                 );
//                 Ok(())
//             }
//             Err(error) => handle_pg_errors(error),
//         }
//     }
// }

// fn handle_pg_errors(error: sqlx::error::Error) -> Result<(), IngestError> {
//     match error {
//         sqlx::error::Error::Database(db_err) => {
//             match db_err.code() {
//                 // 23505 - key is duplicated
//                 // in this case it is acceptable behaviour
//                 Some(std::borrow::Cow::Borrowed("23505")) => {
//                     let pg_err: Box<PgDatabaseError> = db_err.downcast();

//                     tracing::debug!(
//                         table = pg_err.table(),
//                         constraint = pg_err.constraint(),
//                         "data already present details={:?}",
//                         pg_err.detail()
//                     );
//                     Ok(())
//                 }
//                 _ => Err(db_err.into()),
//             }
//         }
//         _ => Err(error.into()),
//     }
// }
