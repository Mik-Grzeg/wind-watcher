use super::super::errors::FetchingError;
use crate::{
    actors::messages::{fetching::FetchNewForecastsMsg, ingesting::WindguruIngestForecastMsg},
    config::Settings,
    types::windguru::{ForecastParamsMetadata, IdModel, IdSpot, Spot, WindguruForecasts},
};
use anyhow::anyhow;
use super::super::Authorizer;
use async_trait::async_trait;
use std::{str::FromStr, sync::Arc};

use super::super::DataFetcher;
use super::super::FetchingClient;
use reqwest::{
    cookie::{Cookie, CookieStore, Jar},
    Client, ClientBuilder, Url,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::BTreeMap, fmt::Display};
use std::{collections::HashMap, time::Duration};
use tracing::instrument;

const WINDGURUR_REFERER: &str = "https://www.windguru.cz";

impl FetchingClient {
    async fn get_forecast_spot_metadata(
        &self,
        spot: IdSpot,
    ) -> Result<ForecastSpotResponse, FetchingError> {
        let url = format!("{}/int/iapi.php", self.url);

        let query_params = ForecastSpotQueryParams {
            id_spot: spot,
            method: WindguruMethod::ForecastSpot,
        };

        let forecast_spot_response = self
            .client
            .get(url)
            .header("Referer", WINDGURUR_REFERER)
            .query(&query_params)
            .send()
            .await?;

        let response_status = forecast_spot_response.status().as_u16();
        let forecast_metadata = forecast_spot_response.json().await?;

        Ok(forecast_metadata)
    }

    async fn get_forecast_data(
        &self,
        forecast_query_params: &ForecastQueryParams,
    ) -> Result<WindguruForecasts, FetchingError> {
        let url = format!("{}/int/iapi.php", self.url);

        let forecast_response = self
            .client
            .get(url)
            .header("Referer", WINDGURUR_REFERER)
            .query(&forecast_query_params)
            .send()
            .await?;

        let response_status = forecast_response.status().as_u16();
        tracing::debug!(response_status = response_status, "Fetching forecast");

        let forecasts = forecast_response.json().await?;

        Ok(forecasts)
    }
}

impl From<&Settings> for FetchingClient {
    fn from(settings: &Settings) -> Self {
        FetchingClient::new(settings.windguru.url.clone())
    }
}

#[async_trait]
impl DataFetcher for FetchingClient {
    type InMessage = FetchNewForecastsMsg;
    type OutMessage = WindguruIngestForecastMsg;

    #[instrument(skip(self))]
    async fn fetch_forecast(
        &self,
        params: FetchNewForecastsMsg,
    ) -> Result<WindguruIngestForecastMsg, FetchingError> {
        self.authorize().await?;

        let ForecastSpotResponse { models, spots } =
            self.get_forecast_spot_metadata(params.spot).await?;

        let spot = Spot::try_from(spots)?;
        let forecast_query_params = BTreeMap::<IdModel, ForecastQueryParams>::from(models);
        let forecast = self
            .get_forecast_data(forecast_query_params.get(&3).unwrap())
            .await?;

        Ok(WindguruIngestForecastMsg { forecast, spot })
    }
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum WindguruMethod {
    Forecast,
    ForecastSpot,
}

#[derive(Serialize)]
struct ForecastSpotQueryParams {
    #[serde(rename(serialize = "q"))]
    pub method: WindguruMethod,
    pub id_spot: IdSpot,
}

#[derive(Serialize)]
struct ForecastQueryParams {
    pub rundef: String,
    pub initstr: String,
    #[serde(flatten)]
    pub forecast_spot: ForecastSpotQueryParams,
    pub id_model: IdModel,
    #[serde(rename(serialize = "WGCACHEABLE"))]
    pub wgcacheable: u32,
    pub cachefix: String,
}

#[derive(Deserialize, Debug)]
struct ForecastSpotResponse {
    #[serde(flatten)]
    pub models: ForecastMetadataWrapper,
    #[serde(rename = "spots")]
    pub spots: HashMap<String, Spot>,
}

#[derive(Deserialize, Debug)]
struct ForecastMetadataWrapper {
    #[serde(rename = "tabs")]
    pub models: Vec<ForecastMetadata>,
}

#[derive(Deserialize, Debug)]
struct ForecastMetadata {
    id_spot: IdSpot,
    id_model: IdModel,
    #[serde(rename = "id_model_arr")]
    params: Vec<ForecastParamsMetadata>,
    options: HashMap<String, Value>,
}

impl From<ForecastMetadataWrapper> for BTreeMap<IdModel, ForecastQueryParams> {
    fn from(response: ForecastMetadataWrapper) -> Self {
        response
            .models
            .into_iter()
            .filter_map(|model| {
                let Some(model_metadata) = model
                    .params
                    .into_iter()
                    .find(|model_metadata| {
                        model_metadata.id_model == model.id_model
                    }) else { return None };

                Some((
                    model.id_model,
                    ForecastQueryParams {
                        id_model: model.id_model,
                        forecast_spot: ForecastSpotQueryParams {
                            id_spot: model.id_spot,
                            method: WindguruMethod::Forecast,
                        },
                        rundef: model_metadata.rundef,
                        initstr: model_metadata.initstr,
                        cachefix: model_metadata.cachefix,
                        wgcacheable: 21600,
                    },
                ))
            })
            .collect::<BTreeMap<IdModel, ForecastQueryParams>>()
    }
}
