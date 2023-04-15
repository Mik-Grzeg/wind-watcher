use lib::config::init_config;

use lib::logging::init_logger;
use lib::state::State;

#[cfg(feature = "lambda")]
use aws_lambda_events::event::cloudwatch_events::CloudWatchEvent;

async fn start() {
    let settings = init_config();
    tracing::info!("Starting forecast_ingester with settings: {}", settings);

    State::start(settings).await;
}

#[cfg(feature = "lambda")]
async fn function_handler(event: LambdaEvent<CloudWatchEvent>) -> Result<(), Error> {
    start().await;
    Ok(())
}

#[tokio::main]
#[cfg(feature = "lambda")]
async fn main() -> Result<(), Error> {
    init_logger();

    run(service_fn(function_handler)).await
}

#[actix::main]
#[cfg(not(feature = "lambda"))]
async fn main() {
    init_logger();

    start().await;
}
