use super::super::errors::FetchError;
use super::super::authorization::Authorizer;
use crate::{
    actors::messages::{fetching::WindguruForecastFetchMsg, ingesting::WindguruIngestForecastMsg},
    config::Settings,
    types::windguru::{ForecastParamsMetadata, IdModel, IdSpot, Spot, WindguruForecasts},
};

use async_trait::async_trait;

use super::super::DataFetcher;
use super::super::FetchingClient;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::collections::HashMap;
use tracing::instrument;

const WINDGURUR_REFERER: &str = "https://www.windguru.cz";

impl FetchingClient {
    async fn get_forecast_spot_metadata(
        &self,
        spot: IdSpot,
    ) -> Result<ForecastSpotResponse, FetchError> {
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

        let _response_status = forecast_spot_response.status().as_u16();
        let forecast_metadata = forecast_spot_response.json().await?;

        Ok(forecast_metadata)
    }

    async fn get_forecast_data(
        &self,
        forecast_query_params: &ForecastQueryParams,
    ) -> Result<WindguruForecasts, FetchError> {
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
    type InMessage = WindguruForecastFetchMsg;
    type OutMessage = WindguruIngestForecastMsg;

    #[instrument(skip(self))]
    async fn fetch_forecast(
        &self,
        params: Self::InMessage,
    ) -> Result<Self::OutMessage, FetchError> {
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

    #[serde(rename = "options")]
    _options: HashMap<String, Value>,
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
