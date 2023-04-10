use std::path::Path;

use lib::config::ConfigCache;
use lib::config::DataStorage;
use sqlx::{migrate::Migrator, Pool, Postgres};

fn init_config() -> DataStorage {
    ConfigCache::new().into::<DataStorage>()
}

#[tokio::main]
async fn main() {
    let config = init_config();

    match config {
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
        }
    }
}
