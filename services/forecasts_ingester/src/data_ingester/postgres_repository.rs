use crate::{
    actors::messages::ingesting::WindguruIngestForecastMsg,
    types::windguru::{Spot, WindguruForecasts},
};

use super::DataIngester;
use async_trait::async_trait;
use sqlx::{postgres::PgQueryResult, PgPool, QueryBuilder};

#[async_trait]
impl DataIngester for PgPool {
    async fn ingest_forecast(&self, data: WindguruIngestForecastMsg) -> Result<(), anyhow::Error> {
        let WindguruIngestForecastMsg { forecast, spot } = data;
        
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
        query_builder.push_values(forecast.forecasts, |mut b, fcst| {
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

        sqlx::query("INSERT INTO spots (id, name, country, models) VALUES ($1, $2, $3, $4) ON CONFLICT (id) DO NOTHING")
            .bind(spot.id)
            .bind(spot.name)
            .bind(spot.country)
            .bind(spot.models)
            .execute(self).await?;

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
            Err(error) => {
                tracing::error!(
                    error = error.to_string(),
                    "failed to save data in postgres storage"
                );
                Err(error.into())
            }
        }
    }
}
