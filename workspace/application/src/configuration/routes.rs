use std::{
    // time::Duration,
    sync::Arc, };

use crate::{
    handlers::stub_entity_handler::{
        add_stub_entity_handler, get_stub_entity_handler, list_stub_entity_handler,
        update_stub_entity_handler,
    },
    middleware::request_middleware::RequestLayer,
};

use axum::{
    // error_handling::HandleErrorLayer,
    // http::StatusCode,
    routing::{get, post, put},
    Router,
};
use tower::{
    // buffer::BufferLayer, 
    // limit::RateLimitLayer, 
    ServiceBuilder};

use super::app_state::AppState;

pub async fn build_routes(state: Arc<AppState>) -> Router {
    let middleware_stacks = ServiceBuilder::new()
        // .layer(HandleErrorLayer::new(|_| async move {
        //     (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
        // }))
        // .layer(BufferLayer::new(1024))
        // .layer(RateLimitLayer::new(1, Duration::from_secs(60)))
        .layer(RequestLayer);

    Router::new()
        .route("/api/v1/stub-entity", get(list_stub_entity_handler))
        .route("/api/v1/stub-entity/:id", get(get_stub_entity_handler))
        .route("/api/v1/stub-entity", post(add_stub_entity_handler))
        .route("/api/v1/stub-entity/:id", put(update_stub_entity_handler))
        .layer(middleware_stacks)
        .with_state(state)
}
