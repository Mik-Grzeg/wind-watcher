use std::path::Path;

use lib::config::ConfigCache;
use lib::config::DataStorage;
use lib::logging::init_logger;
use serde::Deserialize;
use sqlx::{migrate::Migrator, Pool, Postgres};

#[derive(Deserialize)]
struct Config {
    pub storage: DataStorage,
}

fn init_config() -> Config {
    ConfigCache::default().into::<Config>()
}

#[tokio::main]
async fn main() {
    init_logger();
    let config = init_config();

    match config.storage {
        DataStorage::S3(_config) => unimplemented!(),
        DataStorage::Postgresql(config) => {
            let migrations_path = Path::new("./migrations");
            let migrator = Migrator::new(migrations_path)
                .await
                .expect("unable to creat migrator}");

            let pool = Pool::<Postgres>::connect(&config.connection_url)
                .await
                .unwrap();
            migrator.run(&pool).await.unwrap();
            tracing::info!("Storage schema migration was successfull");
        }
    }
}
