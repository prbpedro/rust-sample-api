use tokio::task_local;

#[derive(Clone)]
pub struct RequestData {
    pub correlation_id: String,
}
task_local! {
    pub static REQUEST_DATA: RequestData;
}


#[macro_export]
macro_rules! log_with_span {
    ($level:expr, $message:expr) => {
        use opentelemetry::trace::TraceContextExt;
        use tracing::Level;
        use tracing_opentelemetry::OpenTelemetrySpanExt;
        use infrastructure::logging::logging_util::REQUEST_DATA;

        {
            let current_span = tracing::Span::current();
            let context = current_span.context();
            let otel_span = context.span();
            let span_id = otel_span.span_context().span_id().to_string();
            let trace_id = otel_span.span_context().trace_id().to_string();
            let correlation_id = REQUEST_DATA
            .try_with(|data| data.correlation_id.clone())
            .unwrap_or_else(|_| "none".to_string());
                    let file = file!();
            let line = line!();

            match $level {
                tracing::Level::TRACE => tracing::trace!(span_id = %span_id, trace_id = %trace_id, correlation_id = %correlation_id, file = %file, line = %line, $message),
                tracing::Level::DEBUG => tracing::debug!(span_id = %span_id, trace_id = %trace_id, correlation_id = %correlation_id, file = %file, line = %line, $message),
                tracing::Level::INFO => tracing::info!(span_id = %span_id, trace_id = %trace_id, correlation_id = %correlation_id, file = %file, line = %line, $message),
                tracing::Level::WARN => tracing::warn!(span_id = %span_id, trace_id = %trace_id, correlation_id = %correlation_id, file = %file, line = %line, $message),
                tracing::Level::ERROR => tracing::error!(span_id = %span_id, trace_id = %trace_id, correlation_id = %correlation_id, file = %file, line = %line, $message),
            }
        }
    };
}