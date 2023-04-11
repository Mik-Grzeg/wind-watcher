use lib::config::init_config;
use lib::data_fetcher::{windguru::WindguruSpotClient, DataFetcher};
use lib::logging::init_logger;
use lib::state::State;

#[actix::main]
async fn main() {
    init_logger();
    let settings = init_config();

    tracing::info!("Starting forecast_ingester with settings: {}", settings);
    State::start(settings).await;
}
