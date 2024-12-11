use tokio::task_local;

#[derive(Clone)]
pub struct RequestData {
    pub correlation_id: String,
}

task_local! {
    pub static REQUEST_DATA: RequestData;
}

impl RequestData {
    pub fn new(correlation_id: String) -> Self {
        Self { correlation_id }
    }
}





#[macro_export]
macro_rules! log_with_span {
    ($level:expr, $($arg:tt)*) => {
        {
            let current_span = tracing::Span::current();
            let context = current_span.context();
            let otel_span = context.span();
            let span_id = otel_span.span_context().span_id().to_string();
            let trace_id = otel_span.span_context().trace_id().to_string();
            let correlation_id = REQUEST_DATA
            .try_with(|data| data.correlation_id.clone())
            .unwrap_or_else(|_| "none".to_string());

            tracing::event!($level, span_id = %span_id, trace_id = %trace_id, correlation_id = %correlation_id, $($arg)*);
        }
    };
    ($level:expr, { $($field:tt)* }, $($arg:tt)*) => {
        {
            let current_span = tracing::Span::current();
            let context = current_span.context();
            let otel_span = context.span();
            let span_id = otel_span.span_context().span_id().to_string();
            let trace_id = otel_span.span_context().trace_id().to_string();
            let correlation_id = REQUEST_DATA
            .try_with(|data| data.correlation_id.clone())
            .unwrap_or_else(|_| "none".to_string());

            tracing::event!($level, { $($field)* }, span_id = %span_id, trace_id = %trace_id, correlation_id = %correlation_id, $($arg)*);
        }
    };
}