use super::super::errors::FetchError;
use crate::{
    actors::messages::{
        fetching::WindguruForecastFetchMsg,
        ingesting::{IngestMsg, WindguruForecast},
    },
    config::Settings,
    types::windguru::forecast::{ForecastParamsMetadata, IdModel, IdSpot, Spot, WindguruForecasts},
};

use super::super::client::FetchingClient;

use super::WINDGURU_REFERER;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::collections::HashMap;

async fn get_forecast_spot_metadata(
    fetcher: &FetchingClient,
    spot: IdSpot,
) -> Result<ForecastSpotResponse, FetchError> {
    let url = format!("{}/int/iapi.php", fetcher.url);

    let query_params = ForecastSpotQueryParams {
        id_spot: spot,
        method: WindguruMethod::ForecastSpot,
    };

    let forecast_spot_response = fetcher
        .client
        .get(url)
        .header("Referer", WINDGURU_REFERER)
        .query(&query_params)
        .send()
        .await?;

    let _response_status = forecast_spot_response.status().as_u16();
    let forecast_metadata = forecast_spot_response.json().await?;

    Ok(forecast_metadata)
}

async fn get_forecast_data(
    fetcher: &FetchingClient,
    forecast_query_params: &ForecastQueryParams,
) -> Result<WindguruForecasts, FetchError> {
    let url = format!("{}/int/iapi.php", fetcher.url);

    let forecast_response = fetcher
        .client
        .get(url)
        .header("Referer", WINDGURU_REFERER)
        .query(&forecast_query_params)
        .send()
        .await?;

    let response_status = forecast_response.status().as_u16();
    tracing::debug!(response_status = response_status, "Fetching forecast");

    let forecasts = forecast_response.json().await?;

    Ok(forecasts)
}

pub async fn get_forecast(
    fetcher: &FetchingClient,
    params: WindguruForecastFetchMsg,
) -> Result<IngestMsg, FetchError> {
    let ForecastSpotResponse { models, spots } =
        get_forecast_spot_metadata(fetcher, params.spot).await?;

    let spot = Spot::try_from(spots)?;
    let forecast_query_params = BTreeMap::<IdModel, ForecastQueryParams>::from(models);
    let forecast = get_forecast_data(fetcher, forecast_query_params.get(&3).unwrap()).await?;

    Ok(IngestMsg::WindguruForecast(WindguruForecast {
        forecast,
        spot,
    }))
}

impl FetchingClient {}

impl From<&Settings> for FetchingClient {
    fn from(settings: &Settings) -> Self {
        FetchingClient::new(settings.windguru.url.clone())
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
