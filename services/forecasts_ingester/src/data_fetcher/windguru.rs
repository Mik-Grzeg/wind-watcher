use crate::{
    config::Settings,
    types::windguru::{ForecastParamsMetadata, IdModel, IdSpot, WindguruForecast},
};
use anyhow::anyhow;
use async_trait::async_trait;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::{collections::HashMap, time::Duration};
use tracing::instrument;

use super::DataFetcher;
use reqwest::{Client, ClientBuilder};

const WINDGURUR_REFERER: &str = "https://www.windguru.cz";

#[derive(Debug)]
pub struct WindguruSpotClient {
    client: Client,
    url: String,
    spot_id: u32,
}

impl WindguruSpotClient {
    pub fn new(url: String, spot_id: u32) -> Self {
        let client = ClientBuilder::new()
            .connect_timeout(Duration::from_secs(30))
            .cookie_store(true)
            .build()
            .unwrap();

        Self {
            client,
            url,
            spot_id,
        }
    }

    async fn get_cookies(&self) -> Result<String, reqwest::Error> {
        let url = format!("{}/{}", self.url, self.spot_id);

        // Get authorization cookies
        let cookie_response = self.client.get(url).send().await?;

        let cookies = cookie_response
            .cookies()
            .map(|cookie| format!("{}={}", cookie.name(), cookie.value()))
            .collect::<Vec<String>>()
            .join("; ");

        tracing::debug!(
            cookies = cookies,
            status = cookie_response.status().as_u16(),
            "Fetching auth cookies"
        );
        Ok(cookies)
    }

    async fn get_forecast_spot_metadata(
        &self,
        cookies: &str,
    ) -> Result<ForecastSpotResponse, reqwest::Error> {
        let url = format!("{}/int/iapi.php", self.url);

        let query_params = ForecastSpotQueryParams {
            id_spot: self.spot_id,
            method: WindguruMethod::ForecastSpot,
        };

        let forecast_spot_response = self
            .client
            .get(url)
            .header("Referer", WINDGURUR_REFERER)
            .header("Cookie", cookies)
            .query(&query_params)
            .send()
            .await?;

        let response_status = forecast_spot_response.status().as_u16();
        let forecast_metadata = forecast_spot_response.json().await?;

        tracing::debug!(response_status = response_status, "Fetching model metadata");
        Ok(forecast_metadata)
    }

    async fn get_forecast_data(
        &self,
        cookies: &str,
        forecast_query_params: &ForecastQueryParams,
    ) -> Result<WindguruForecast, reqwest::Error> {
        let url = format!("{}/int/iapi.php", self.url);

        let forecast_response = self
            .client
            .get(url)
            .header("Referer", WINDGURUR_REFERER)
            .header("Cookie", cookies)
            .query(&forecast_query_params)
            .send()
            .await?;

        let response_status = forecast_response.status().as_u16();
        tracing::debug!(response_status = response_status, "Fetching forecast");

        let forecast = forecast_response.json().await?;

        Ok(forecast)
    }
}

impl From<Settings> for WindguruSpotClient {
    fn from(settings: Settings) -> Self {
        WindguruSpotClient::new(settings.windguru_url, 36048)
    }
}

#[async_trait]
impl DataFetcher<WindguruForecast> for WindguruSpotClient {
    #[instrument(skip(self))]
    async fn fetch_forecast(&self) -> Result<WindguruForecast, anyhow::Error> {
        let cookies = self
            .get_cookies()
            .await
            .map_err(|err| anyhow!("Unable to fetch cookies err={}", err))?;

        let forecast_spot_metadata = self
            .get_forecast_spot_metadata(&cookies)
            .await
            .map_err(|err| anyhow!("Unable to fetch metadata err={}", err))?;

        let forecast_query_params =
            BTreeMap::<IdModel, ForecastQueryParams>::from(forecast_spot_metadata);
        let forecast = self
            .get_forecast_data(&cookies, forecast_query_params.get(&3).unwrap())
            .await
            .map_err(|err| anyhow!("Unable to fetch data err={}", err))?;

        Ok(forecast)
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
    #[serde(rename = "tabs")]
    pub models: Vec<ForecastSpotResponseInner>,
}

#[derive(Deserialize, Debug)]
struct ForecastSpotResponseInner {
    id_spot: IdSpot,
    id_model: IdModel,
    #[serde(rename = "id_model_arr")]
    params: Vec<ForecastParamsMetadata>,
    options: HashMap<String, Value>,
}

impl From<ForecastSpotResponse> for BTreeMap<IdModel, ForecastQueryParams> {
    fn from(response: ForecastSpotResponse) -> Self {
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
