use super::FORMAT;
use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};

type IdStation = i32;

#[derive(Serialize)]
pub struct WindguruStationFetchParams {
    pub id_station: IdStation,
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
    pub avg_minutes: u32,
    #[serde(rename = "q")]
    pub method: String,
    pub graph_info: u32,
}

impl Default for WindguruStationFetchParams {
    fn default() -> Self {
        WindguruStationFetchParams {
            id_station: Default::default(),
            from: Default::default(),
            to: Default::default(),
            avg_minutes: 5,
            method: "station_data".into(),
            graph_info: 1,
        }
    }
}

#[derive(Debug)]
pub struct WindguruStationReading {
    pub datetime_local: DateTime<FixedOffset>,
    pub gustiness: Option<f32>,
    pub temperature: Option<f32>,
    pub wind_avg: Option<f32>,
    pub wind_max: Option<f32>,
    pub wind_min: Option<f32>,
    pub wind_direction: Option<i32>,
    pub relative_humidity: Option<i32>,
    pub mean_sea_level_pressure: Option<f32>,
}

#[derive(Debug)]
pub struct WindguruStationReadingsWithTime {
    pub tzoffset: FixedOffset,
    pub readings: Vec<WindguruStationReading>,
}

#[derive(Deserialize, Debug)]
pub struct WindguruStationData {
    #[serde(with = "station_arrays_format", flatten)]
    pub readings: WindguruStationReadingsWithTime,

    #[serde(rename = "startstamp", deserialize_with = "from_unixstamp")]
    pub datetime_start_utc: DateTime<Utc>,

    #[serde(rename = "endstamp", deserialize_with = "from_unixstamp")]
    pub datetime_end_utc: DateTime<Utc>,
}

fn from_unixstamp<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let t: i64 = Deserialize::deserialize(deserializer)?;
    let naive_datetime = NaiveDateTime::from_timestamp_opt(t, 0)
        .ok_or(anyhow::anyhow!(
            "unable to create datetime from unix timestamp"
        ))
        .map_err(serde::de::Error::custom)?;
    Ok(DateTime::from_utc(naive_datetime, Utc))
}

mod station_arrays_format {
    use chrono::NaiveDateTime;

    use super::*;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<WindguruStationReadingsWithTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data_map: serde_json::Map<String, serde_json::Value> =
            serde_json::Map::deserialize(deserializer)?;

        let num_data_points = data_map["datetime"].as_array().unwrap().len();

        let mut readings = Vec::new();

        let tzoffset =
            FixedOffset::west_opt(data_map["tzoffset"].as_i64().map(|val| val as i32).unwrap())
                .unwrap();
        for i in 0..num_data_points {
            let datetime_local: DateTime<FixedOffset> = DateTime::from_local(
                NaiveDateTime::parse_from_str(data_map["datetime"][i].as_str().unwrap(), FORMAT)
                    .map_err(serde::de::Error::custom)?,
                tzoffset,
            );
            let gustiness = data_map["gustiness"][i].as_f64().map(|val| val as f32);
            let temperature = data_map["temperature"][i].as_f64().map(|val| val as f32);
            let wind_avg = data_map["wind_avg"][i].as_f64().map(|val| val as f32);
            let wind_max = data_map["wind_max"][i].as_f64().map(|val| val as f32);
            let wind_min = data_map["wind_min"][i].as_f64().map(|val| val as f32);
            let wind_direction = data_map["wind_direction"][i].as_i64().map(|val| val as i32);
            let relative_humidity = data_map["rh"][i].as_i64().map(|val| val as i32);
            let mean_sea_level_pressure = data_map["mslp"][i].as_f64().map(|val| val as f32);

            readings.push(WindguruStationReading {
                datetime_local,
                gustiness,
                temperature,
                wind_avg,
                wind_max,
                wind_min,
                wind_direction,
                relative_humidity,
                mean_sea_level_pressure,
            })
        }

        Ok(WindguruStationReadingsWithTime { tzoffset, readings })
    }
}

#[cfg(test)]
mod tests {
    use super::WindguruStationData;

    #[test]
    fn deserialize_proper_windsurfing_data() {
        let json_data = create_test_proper_windguru_station_data();
        let jd = &mut serde_json::Deserializer::from_str(json_data);

        let result: Result<WindguruStationData, _> = serde_path_to_error::deserialize(jd);

        let _data = match result {
            Ok(data) => data,
            Err(err) => {
                panic!("err_for_key={} details={}", err.path(), err)
            }
        };
    }

    fn create_test_proper_windguru_station_data() -> &'static str {
        r#"
{
  "datetime": [
    "2023-04-15 07:30:00",
    "2023-04-15 07:36:00"
  ],
  "wind_avg": [
    23.7,
    22.2
  ],
  "wind_max": [
    35.1,
    35.7
  ],
  "wind_min": [
    null,
    null
  ],
  "wind_direction": [
    155,
    160
  ],
  "temperature": [
    16.6,
    16.6
  ],
  "mslp": [
    1015,
    1015
  ],
  "rh": [
    84,
    84
  ],
  "gustiness": [
    null,
    null
  ],
  "sunrise": "06:46",
  "sunset": "19:50",
  "startstamp": 1681533012,
  "endstamp": 1681597812,
  "tzoffset": 10800
}
        "#
    }
}
