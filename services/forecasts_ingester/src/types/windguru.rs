use std::{collections::HashMap, time::Duration};

use chrono::{DateTime, Utc};
use serde::{de::Visitor, Deserialize};
use serde_json::Value;
use serde_with::{serde_as, DurationSeconds};

#[derive(Deserialize, Debug)]
struct ForecastSpotModelMetadata {}

pub type IdSpot = u32;
pub type IdModel = u32;

#[derive(Debug)]
pub struct ForecastModelSpotMetadata {
    params: ForecastParamsMetadata,
    id_spot: IdSpot,
    options: HashMap<String, Value>,
}

#[derive(Deserialize, Debug)]
pub struct ForecastParamsMetadata {
    pub id_model: IdModel,
    pub initstr: String,
    pub rundef: String,
    pub period: u32,
    pub cachefix: String,
}

#[derive(Deserialize, Debug)]
pub struct WindguruForecast {
    id_spot: IdSpot,
    wgmodel: WgModel,
    sunrise: String,
    sunset: String,
    fcst: Fcst,
}

#[derive(Deserialize, Debug)]
pub struct WgModel {
    id_model: IdModel,
    model: String,
    model_name: String,
    #[serde(with = "windguru_wg_format")]
    initdate: DateTime<Utc>,
    hr_start: u32,
    hr_end: u32,
    hr_step: u32,
    wave: bool,
    rundef: String,
}

enum ForecastValue {
    Null,
}

#[serde_with::serde_as]
#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "UPPERCASE"))]
struct Fcst {
    gust: Vec<Option<f32>>,
    flhgt: Vec<Option<u32>>,
    slp: Vec<Option<u32>>,
    rh: Vec<Option<u32>>,
    tcdc: Vec<Option<u32>>,
    apcp: Vec<Option<f32>>,
    apcp1: Vec<Option<f32>>,
    hcdc: Vec<Option<u32>>,
    lcdc: Vec<Option<u32>>,
    windspd: Vec<Option<f32>>,
    winddir: Vec<Option<u32>>,
    slhgt: Vec<Option<u32>>,
    pcpt: Vec<Option<u32>>,
    tmpe: Vec<Option<f32>>,
    #[serde(rename(deserialize = "hours"))]
    hours: Vec<Option<u32>>,
    #[serde(rename(deserialize = "initstamp"))]
    #[serde_as(as = "DurationSeconds<u64>")]
    initstamp: Duration,
}

mod windguru_wg_format {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{Deserialize, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}
