use crate::{actors::messages::ingesting::{IngestMsg, WindguruForecast}, types::windguru::station::WindguruStationData};

use super::errors::IngestError;
use super::DataIngester;
use anyhow::anyhow;

use async_trait::async_trait;
use sqlx::{postgres::PgDatabaseError, PgPool, QueryBuilder};

#[async_trait]
impl DataIngester for PgPool {
    async fn ingest_forecast(&self, data: IngestMsg) -> Result<(), IngestError> {
        match data {
            IngestMsg::WindguruForecast(WindguruForecast { forecast }) => {
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
                query_builder.push_values(Vec::clone(&forecast.forecasts), |mut b, fcst| {
                    b.push_bind(forecast.id_spot)
                        .push_bind(forecast.wgmodel.id_model)
                        .push_bind(fcst.forecast_from)
                        .push_bind(fcst.forecast_for)
                        .push_bind(forecast.wgmodel.wave)
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

                sqlx::query(
                    "INSERT INTO models (id, identifier, name) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
                )
                .bind(forecast.wgmodel.id_model)
                .bind(forecast.wgmodel.model)
                .bind(forecast.wgmodel.model_name)
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
            IngestMsg::WindguruStationReading(id_station, WindguruStationData{ readings, datetime_start_utc, datetime_end_utc })  => {
                let mut query_builder = QueryBuilder::new(
                    r#"INSERT INTO station_readings(
                        id_spot,
                        time,
                        -- sunrise, to be added later on
                        -- sunset, to be added later on
                        wind_speed_avg,
                        wind_max,
                        wind_direction,
                        temperature
                    ) "#,
                );

                query_builder.push_values(readings.readings, |mut b, reading| {
                    b.push_bind(id_station)
                        .push_bind(reading.datetime_local)
                        .push_bind(reading.wind_avg)
                        .push_bind(reading.wind_max)
                        .push_bind(reading.wind_direction)
                        .push_bind(reading.temperature);
                });

                match query_builder.build().execute(self).await {
                    Ok(rows_affected) => {
                        tracing::debug!(
                            rows_affected = rows_affected.rows_affected(),
                            "sucessfuly inserted data to postgres storage"
                        );
                        Ok(())
                    }
                    Err(error) => {
                        handle_pg_errors(error)
                    }
                }
            },
            IngestMsg::WindguruSpot(spot) => {
                sqlx::query("INSERT INTO spots (id, name, country, models, gmt_hour_offset) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (id) DO NOTHING")
                    .bind(spot.id)
                    .bind(spot.name)
                    .bind(spot.country)
                    .bind(spot.models)
                    .bind(spot.gmt_hour_offset)
                    .execute(self).await?;
                Ok(())
            },
            other => {
                tracing::warn!("Unimplemented message received {other}");
                Err(anyhow!("Unimplemented message received {other}").into())
            }
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
