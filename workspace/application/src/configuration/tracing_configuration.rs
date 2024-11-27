use tracing::Level;
use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};

pub fn configure_tracing() {    
    tracing_subscriber::fmt()
        .json()
        .with_max_level(Level::INFO)
        .with_span_events(FmtSpan::FULL)
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}