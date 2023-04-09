use std::fmt::Display;

use config::Config;
use serde::Deserialize;

pub fn init_config() -> Settings {
    Settings::from(ConfigCache::new())
}

struct ConfigCache {
    config: Config,
}

impl ConfigCache {
    pub fn new() -> Self {
        let config = Config::builder()
            .add_source(config::File::with_name("Settings.toml").required(false))
            .add_source(config::Environment::with_prefix("RUSTAPP_"))
            .build()
            .unwrap();

        Self { config }
    }
}

impl From<ConfigCache> for Settings {
    fn from(cache: ConfigCache) -> Self {
        cache.config.try_deserialize().unwrap()
    }
}

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub windguru_url: String,
}

impl Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}
