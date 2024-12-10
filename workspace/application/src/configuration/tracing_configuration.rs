use opentelemetry::{global, trace::TracerProvider};
use opentelemetry_sdk::propagation::TraceContextPropagator;
use tracing_subscriber::{
    fmt::{self},
    layer::SubscriberExt,
    prelude::*,
    EnvFilter, Layer,
};

pub fn configure_tracing() {
    global::set_text_map_propagator(TraceContextPropagator::new());

    let tracer_provider = opentelemetry_sdk::trace::TracerProvider::builder()
        .with_batch_exporter(
            opentelemetry_otlp::SpanExporter::builder()
                .with_tonic()
                .build()
                .unwrap(),
            opentelemetry_sdk::runtime::Tokio,
        )
        .build();
    let tracer = tracer_provider.tracer("tracing");

    let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    let fmt_layer = fmt::layer()
        .json()
        .with_current_span(true)
        .with_span_list(true)
        .with_filter(EnvFilter::from_default_env());

    let filter_layer = EnvFilter::try_from_default_env().unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(otel_layer)
        .init();
}
