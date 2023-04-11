use std::fmt::Display;

use config::Config;
use serde::Deserialize;

use crate::types::windguru::WindguruConfig;

pub fn init_config() -> Settings {
    ConfigCache::new().into::<Settings>()
}

pub struct ConfigCache {
    config: Config,
}

impl ConfigCache {
    pub fn new() -> Self {
        let config = Config::builder()
            .add_source(config::File::with_name("Settings.toml").required(false))
            .add_source(
                config::Environment::with_prefix("RUSTAPP")
                    .try_parsing(true)
                    .separator("__"),
            )
            .build()
            .unwrap();

        Self { config }
    }

    pub fn into<'de, T: Deserialize<'de>>(self) -> T {
        self.config.try_deserialize().unwrap()
    }
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", content = "config")]
#[serde(rename_all = "lowercase")]
pub enum DataStorage {
    S3(S3Config),
    Postgresql(PostgresqlConfig),
}

#[derive(Deserialize, Debug)]
pub struct S3Config {}

#[derive(Deserialize, Debug)]
pub struct PostgresqlConfig {
    pub connection_url: String,
}

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub windguru: WindguruConfig,
    pub storage: DataStorage,
}

impl Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}
