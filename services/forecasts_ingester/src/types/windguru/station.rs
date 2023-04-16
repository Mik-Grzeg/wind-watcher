use super::FORMAT;
use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Deserializer, Serialize};

type IdStation = i32;

#[derive(Serialize)]
pub struct WindguruStationFetchParams<'a> {
    station: IdStation,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
    avg_minutes: u32,
    #[serde(rename = "q")]
    method: &'a str,
    graph_info: u32,
}

impl<'a> Default for WindguruStationFetchParams<'a> {
    fn default() -> Self {
        WindguruStationFetchParams {
            station: Default::default(),
            from: Default::default(),
            to: Default::default(),
            avg_minutes: 5,
            method: "station_data",
            graph_info: 1,
        }
    }
}

pub struct WindguruStationReading {
    pub datetimes_local: DateTime<Utc>,
    pub gustiness: Option<f32>,
    pub temperature: Option<f32>,
    pub wind_avg: Option<f32>,
    pub wind_max: Option<f32>,
    pub wind_min: Option<f32>,
    pub wind_direction: Option<i32>,
    pub relative_humidity: Option<i32>,
    pub mean_sea_level_pressure: Option<f32>,
}

#[derive(Deserialize)]
pub struct WindguruStationData {
    #[serde(with = "station_arrays_format")]
    #[serde(flatten)]
    pub reading: Vec<WindguruStationReading>,

    pub tzoffset: i32,

    #[serde(rename = "startstamp")]
    pub datetime_start_utc: DateTime<Utc>,

    #[serde(rename = "endstamp")]
    pub datetime_end_utc: DateTime<Utc>,
}

mod station_arrays_format {
    use chrono::NaiveDateTime;

    use super::*;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<WindguruStationReading>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data_map: serde_json::Map<String, serde_json::Value> =
            serde_json::Map::deserialize(deserializer)?;

        let num_data_points = data_map["datetime"].as_array().unwrap().len();

        let offset =
            FixedOffset::west_opt(data_map["tzoffset"].as_i64().map(|val| val as i32).unwrap())
                .unwrap();
        for i in 0..num_data_points {
            let _datetime: DateTime<FixedOffset> = DateTime::from_local(
                NaiveDateTime::parse_from_str(data_map["datetime"][i].as_str().unwrap(), FORMAT)
                    .map_err(serde::de::Error::custom)?,
                offset,
            );
            let _gustiness = data_map["gustiness"][i].as_f64().map(|val| val as f32);
            let _temperature = data_map["temperature"][i].as_f64().map(|val| val as f32);
            let _wind_avg = data_map["wind_avg"][i].as_f64().map(|val| val as f32);
            let _wind_max = data_map["wind_max"][i].as_f64().map(|val| val as f32);
            let _wind_min = data_map["wind_min"][i].as_f64().map(|val| val as f32);
            let _wind_direction = data_map["wind_direction"][i].as_i64().map(|val| val as i32);
            let _relative_humidity = data_map["rh"][i].as_i64().map(|val| val as i32);
            let _mean_sea_level_pressure = data_map["mslp"][i].as_f64().map(|val| val as f32);
        }
        unimplemented!()
    }
}
