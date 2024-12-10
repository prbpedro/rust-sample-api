use anyhow::Result;
use axum::{extract::Request, response::Response};
use futures_util::future::BoxFuture;
use infrastructure::log_with_span;
use infrastructure::logging::logging_util::RequestData;
use infrastructure::logging::logging_util::REQUEST_DATA;
use opentelemetry::trace::TraceContextExt;
use std::task::{Context, Poll};
use tower::{Layer, Service};
use tracing::field;
use tracing::info_span;
use tracing::Instrument;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use uuid::Uuid;

#[derive(Clone)]
pub struct TracingLayer;

impl<S> Layer<S> for TracingLayer {
    type Service = TracingMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        TracingMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct TracingMiddleware<S> {
    inner: S,
}

impl<S> Service<Request> for TracingMiddleware<S>
where
    S: Service<Request, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        let method = request.method().clone();
        let path = request.uri().path().to_string();
        let correlation_id = request
                    .headers()
                    .get("x-correlation-id")
                    .and_then(|value| value.to_str().ok())
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| Uuid::new_v4().to_string());

        let span = info_span!(
            "request", 
            method = %method, 
            path = %path, 
            status = field::Empty, 
            trace_id = field::Empty, 
            span_id = field::Empty, 
            correlation_id = %correlation_id);

        let span_clone = span.clone();

        let trace_id = span.context().span().span_context().trace_id().to_string();
        let span_id = span.context().span().span_context().span_id().to_string();
        span.record("trace_id", &trace_id);
        span.record("span_id", &span_id);

        let request_data = RequestData { correlation_id: correlation_id.clone() };


        let future = self.inner.call(request);
        Box::pin(
            REQUEST_DATA.scope(request_data, async move {
                let mut response = future.await?;
                span.record("status", &response.status().as_u16());
                
                let mut level = Level::INFO;
                if !response.status().is_success() {
                    span.record("error", true);
                    level = Level::ERROR;
                }

                log_with_span!(level, "request completed");
                
                response.headers_mut().insert(
                    "x-correlation-id",
                    correlation_id.parse().unwrap(),
                );

                Ok(response)
            }
            .instrument(span_clone)),
        )
    }
}