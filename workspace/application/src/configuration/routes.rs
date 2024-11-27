use std::sync::Arc;

use crate::handlers::stub_entity_handler::{
    add_stub_entity_handler, get_stub_entity_handler, list_stub_entity_handler,
    update_stub_entity_handler,
};

use axum::{
    routing::{get, post, put},
    Router,
};

use super::app_state::AppState;

pub async fn build_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/v1/stub-entity", get(list_stub_entity_handler))
        .route("/api/v1/stub-entity/:id", get(get_stub_entity_handler))
        .route("/api/v1/stub-entity", post(add_stub_entity_handler))
        .route("/api/v1/stub-entity/:id", put(update_stub_entity_handler))
        .with_state(state)
}
