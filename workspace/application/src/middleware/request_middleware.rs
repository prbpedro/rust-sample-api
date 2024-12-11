use anyhow::Result;
use axum::extract::MatchedPath;
use axum::{extract::Request, response::Response};
use futures_util::future::BoxFuture;
use infrastructure::log_with_span;
use infrastructure::logging::logging_util::RequestData;
use infrastructure::logging::logging_util::REQUEST_DATA;
use opentelemetry::trace::TraceContextExt;
use tracing::Level;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use std::task::{Context, Poll};
use std::time::Instant;
use tower::{Layer, Service};
use tracing::field;
use tracing::info_span;
use tracing::Instrument;
use tracing::Span;
use uuid::Uuid;

#[derive(Clone)]
pub struct RequestLayer;

impl<S> Layer<S> for RequestLayer
where
    S: Service<Request, Response = Response> + Clone + Send + 'static,
{
    type Service = RequestMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RequestMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct RequestMiddleware<S: Clone> {
    inner: S,
}

impl<S> Service<Request> for RequestMiddleware<S>
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
        let correlation_id = retrieve_correlation_id(&request);
        let request_http_method = request.method().to_string();
        let request_path_pattern = request.extensions()
            .get::<MatchedPath>()
            .map(|p| p.as_str().to_string())
            .unwrap_or_else(|| "unknown".to_string());
        let request_path = request.uri().path().to_string();

        let span = start_span(
            &correlation_id, 
            &request_http_method,
            &request_path_pattern,
            &request_path);

        let span_clone: Span = span.clone();

        let future = self.inner.call(request);
        Box::pin(
            REQUEST_DATA.scope(
                RequestData::new(correlation_id.clone()),
                async move {
                    let mut response: axum::http::Response<axum::body::Body> = future.await?;

                    record_span_attributes(&response, span);
                    inject_response_data(&mut response, correlation_id);
                    log_request_processed(
                        &response, 
                        start_time, 
                        &request_http_method,
                        &request_path_pattern,
                        &request_path);
                    Ok(response)
                }
                .instrument(span_clone),
            ),
        )
    }
}

fn retrieve_correlation_id(request: &axum::http::Request<axum::body::Body>) -> String {
    request
        .headers()
        .get("x-correlation-id")
        .and_then(|value| value.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| Uuid::new_v4().to_string())
}

fn start_span(
    correlation_id: &String, 
    request_http_method: &String, 
    request_path_pattern: &String, 
    request_path: &String) -> Span {
    let span = info_span!(
            "ROOT_REQUEST", 
            request.http_method = %request_http_method, 
            request.path_pattern = %request_path_pattern, 
            request.path = %request_path, 
            response.status_code = field::Empty, 
            correlation_id = %correlation_id);

    span
}

fn log_request_processed(
    response: &axum::http::Response<axum::body::Body>, 
    start_time: Instant, 
    request_http_method: &String, 
    request_path_pattern: &String, 
    request_path: &String) {

    let duration = start_time.elapsed();
    let duration_ms = duration.as_millis(); 

    if response.status().is_server_error() {
        log_with_span!(
            Level::ERROR, 
            request.http_method=%request_http_method,
            request.path_pattern=%request_path_pattern,
            request.path=%request_path,
            response.status_code=%response.status().as_u16(), 
            duration_ms=%duration_ms,
            "[HTTP REQUEST PROCESSED]");
    } else {
        log_with_span!(
            Level::INFO,  
            request.http_method=%request_http_method,
            request.path_pattern=%request_path_pattern,
            request.path=%request_path,
            response.status_code=%response.status().as_u16(), 
            duration_ms=%duration_ms,
            "[HTTP REQUEST PROCESSED]");
    }
}

fn record_span_attributes(response: &axum::http::Response<axum::body::Body>, span: Span) {
    if response.status().is_server_error() {
        span.record("error", true);
    }
    span.record("response.status_code", &response.status().as_u16());
}

fn inject_response_data(
    response: &mut axum::http::Response<axum::body::Body>,
    correlation_id: String,
) {
    response
        .headers_mut()
        .insert("x-correlation-id", correlation_id.parse().unwrap());
}
