use opentelemetry::{global, trace::TracerProvider, KeyValue};
use opentelemetry_sdk::{propagation::TraceContextPropagator, Resource};
use tracing_subscriber::{
    fmt::{self},
    layer::SubscriberExt,
    prelude::*,
    EnvFilter, Layer,
};

pub fn configure_tracing() {
    global::set_text_map_propagator(TraceContextPropagator::new());

    let resource = Resource::new(vec![
        KeyValue::new(
            opentelemetry_semantic_conventions::resource::SERVICE_NAME,
            env!("CARGO_PKG_NAME"),
        ),
        KeyValue::new(
            opentelemetry_semantic_conventions::resource::SERVICE_VERSION,
            env!("CARGO_PKG_VERSION"),
        ),
    ]);

    let tracer_provider = opentelemetry_sdk::trace::TracerProvider::builder()
        .with_resource(resource)
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
        .with_current_span(false)
        .with_span_list(false)
        .with_file(true)
        .with_line_number(true)
        .with_filter(EnvFilter::from_default_env());

    let filter_layer = EnvFilter::try_from_default_env().unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(otel_layer)
        .init();
}
