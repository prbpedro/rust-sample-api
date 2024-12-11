use std::time::Instant;

use anyhow::Result;
use axum::extract::MatchedPath;
use axum::{extract::Request, response::Response};
use futures_util::future::BoxFuture;
use std::task::{Context, Poll};
use tower::{Layer, Service};

#[derive(Clone)]
pub struct RequestMetricsLayer;

impl<S> Layer<S> for RequestMetricsLayer
where
    S: Service<Request, Response = Response> + Clone + Send + 'static,
{
    type Service = RequestMetricsMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RequestMetricsMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct RequestMetricsMiddleware<S: Clone> {
    inner: S,
}

impl<S> Service<Request> for RequestMetricsMiddleware<S>
where
    S: Service<Request, Response = Response> + Send + Clone + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        let start_time = Instant::now();
        let request_http_method = request.method().to_string();
        let request_path_pattern = request
            .extensions()
            .get::<MatchedPath>()
            .map(|p| p.as_str().to_string())
            .unwrap_or_else(|| "unknown".to_string());

        let future = self.inner.call(request);
        Box::pin(async move {
            let response: axum::http::Response<axum::body::Body> = future.await?;

            if !request_path_pattern.starts_with("/api") {
                return Ok(response);
            }

            let latency = start_time.elapsed().as_secs_f64();
            let response_status_code = response.status().to_string();

            let labels = [
                ("app.name", env!("CARGO_PKG_NAME").to_string()),
                ("app.version", env!("CARGO_PKG_VERSION").to_string()),
                ("request.http_method", request_http_method),
                ("request.path_pattern", request_path_pattern),
                ("response.status_code", response_status_code),
            ];

            metrics::counter!("http_requests_total", &labels).increment(1);
            metrics::histogram!("http_requests_duration_seconds", &labels).record(latency);
            Ok(response)
        })
    }
}
