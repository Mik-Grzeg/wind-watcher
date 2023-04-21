use tracing_subscriber::EnvFilter;

pub fn init_logger() {
    // Initialize tracing
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Unable to set up tracing subscriber");
}
