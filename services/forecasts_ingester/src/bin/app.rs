use lib::config::init_config;
use lib::data_fetcher::{windguru::WindguruSpotClient, DataFetcher};

fn init_logger() {
    // Initialize tracing
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Unable to set up tracing subscriber");
}

#[tokio::main]
async fn main() {
    init_logger();
    let settings = init_config();

    tracing::info!("Starting forecast_ingester with settings: {}", settings);
    let windguru_spot_client = WindguruSpotClient::from(settings);
    let _data = windguru_spot_client.fetch_forecast().await.unwrap();
}
