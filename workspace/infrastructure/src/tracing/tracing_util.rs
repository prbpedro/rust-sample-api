#[macro_export]
macro_rules! create_correlated_span {
    ($span_level:expr, $span_name:expr) => {{
        let correlation_id = REQUEST_DATA
            .try_with(|data| data.correlation_id.clone())
            .unwrap_or_else(|_| "none".to_string());

        span!(
            $span_level,
            $span_name,
            correlation_id = %correlation_id
        )
    }};
}
