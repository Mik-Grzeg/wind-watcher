use std::collections::HashMap;

use super::windguru_datetime_format;
use anyhow::anyhow;
use chrono::{DateTime, Duration, NaiveDateTime, NaiveTime, Utc};
use serde::{de, Deserialize};

pub type IdSpot = i32;
pub type IdModel = i32;

#[derive(Deserialize, Debug)]
pub struct WindguruConfig {
    pub url: String,
    pub spots: Vec<IdSpot>,
    pub models: Vec<IdModel>,
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
pub struct WindguruForecasts {
    pub id_spot: IdSpot,
    pub wgmodel: WgModel,
    #[serde(with = "windguru_hour_minutes_format")]
    pub sunrise: NaiveTime,
    #[serde(with = "windguru_hour_minutes_format")]
    pub sunset: NaiveTime,
    #[serde(with = "forecasts_arrays_format")]
    #[serde(rename = "fcst")]
    pub forecasts: Vec<Fcst>,
}

#[derive(Deserialize, Debug)]
pub struct WgModel {
    pub id_model: IdModel,
    pub model: String,
    pub model_name: String,
    #[serde(with = "windguru_datetime_format")]
    pub initdate: DateTime<Utc>,
    pub hr_start: u32,
    pub hr_end: u32,
    pub hr_step: u32,
    pub wave: bool,
    pub rundef: String,
}

#[derive(Debug, Clone)]
pub struct Fcst {
    pub gust: Option<f32>,
    pub flhgt: Option<i32>,
    pub slp: Option<i32>,
    pub relative_humidity: Option<i32>,
    pub tcdc: Option<i32>,
    pub apcp: Option<f32>,
    pub apcp1: Option<f32>,
    pub wind_speed: Option<f32>,
    pub wind_direction: Option<i32>,
    pub slhgt: Option<i32>,
    pub precipitation: Option<i32>,
    pub temperature: Option<f32>,
    pub forecast_for: DateTime<Utc>,
    pub forecast_from: DateTime<Utc>,
    pub cloud_cover_high: Option<i32>,
    pub cloud_cover_mid: Option<i32>,
    pub cloud_cover_low: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct Spot {
    #[serde(rename = "id_spot")]
    #[serde(deserialize_with = "deserialize_string_as_numeric")]
    pub id: IdSpot,
    #[serde(rename = "spotname")]
    pub name: String,
    pub country: String,
    pub models: Vec<IdModel>,
}

fn deserialize_string_as_numeric<'de, T: std::str::FromStr, D: de::Deserializer<'de>>(
    deserializer: D,
) -> Result<T, D::Error> {
    let value = String::deserialize(deserializer)?;
    str::parse::<T>(&value)
        .map_err(|_| serde::de::Error::custom(format!("unable to parse value: {value}")))
}

impl TryFrom<HashMap<String, Spot>> for Spot {
    type Error = anyhow::Error;

    fn try_from(map: HashMap<String, Spot>) -> Result<Self, Self::Error> {
        let map_len = map.keys().len();
        let error_msg = if map_len > 1 {
            "spots had more keys than 1"
        } else if map_len == 0 {
            "missing spots"
        } else {
            let mut spots = map.into_values().collect::<Vec<Spot>>();
            return Ok(spots.remove(0));
        };

        Err(anyhow!(error_msg))
    }
}

mod forecasts_arrays_format {
    use super::*;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Fcst>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let data_map: serde_json::Map<String, serde_json::Value> =
            serde_json::Map::deserialize(deserializer)?;

        let num_data_points = data_map["GUST"].as_array().unwrap().len();

        let forecast_from = parse_initstamp(&data_map["initstamp"]);
        let update_from = windguru_datetime_format::deserialize(&data_map["update_last"])
            .map_err(serde::de::Error::custom)?;

        let mut data_vec = Vec::with_capacity(num_data_points);

        for i in 0..num_data_points {
            let gust = data_map["GUST"][i].as_f64().map(|val| val as f32);
            let flhgt = data_map["FLHGT"][i].as_i64().map(|val| val as i32);
            let slp = data_map["SLP"][i].as_i64().map(|val| val as i32);
            let relative_humidity = data_map["RH"][i].as_i64().map(|val| val as i32);
            let tcdc = data_map["TCDC"][i].as_i64().map(|val| val as i32);
            let apcp = data_map["APCP1"][i].as_f64().map(|val| val as f32);
            let apcp1 = data_map["APCP1"][i].as_f64().map(|val| val as f32);
            let cloud_cover_high = data_map["HCDC"][i].as_i64().map(|val| val as i32);
            let cloud_cover_mid = data_map["MCDC"][i].as_i64().map(|val| val as i32);
            let cloud_cover_low = data_map["LCDC"][i].as_i64().map(|val| val as i32);
            let wind_speed = data_map["WINDSPD"][i].as_f64().map(|val| val as f32);
            let wind_direction = data_map["WINDDIR"][i].as_i64().map(|val| val as i32);
            let slhgt = data_map["SLHGT"][i].as_i64().map(|val| val as i32);
            let precipitation = data_map["PCPT"][i].as_i64().map(|val| val as i32);
            let temperature = data_map["TMPE"][i].as_f64().map(|val| val as f32);
            let forecast_for =
                forecast_from + Duration::hours(data_map["hours"][i].as_i64().unwrap());

            let data = Fcst {
                gust,
                flhgt,
                slp,
                relative_humidity,
                tcdc,
                apcp,
                apcp1,
                cloud_cover_high,
                cloud_cover_mid,
                cloud_cover_low,
                wind_speed,
                wind_direction,
                slhgt,
                precipitation,
                temperature,
                forecast_for,
                forecast_from: update_from,
            };

            data_vec.push(data);
        }

        Ok(data_vec)
    }

    #[cfg(test)]
    mod tests {

        #[test]
        fn test_deserialization() {}
    }
}

mod windguru_hour_minutes_format {
    use chrono::NaiveTime;
    use serde::{Deserialize, Deserializer};

    const FORMAT: &str = "%H:%M";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

fn parse_initstamp(initstamp: &serde_json::Value) -> DateTime<Utc> {
    let naive_datetime = NaiveDateTime::from_timestamp_opt(initstamp.as_i64().unwrap(), 0).unwrap();
    DateTime::from_utc(naive_datetime, Utc)
}
