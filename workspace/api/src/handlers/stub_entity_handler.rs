use std::sync::Arc;

use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use axum_extra::extract::WithRejection;
use infrastructure::{create_correlated_span, log_with_span};
use infrastructure::logging::logging_task_local::REQUEST_DATA;
use opentelemetry::trace::TraceContextExt;
use serde_json::{json, Value};
use tracing::{Level, span};
use tracing_opentelemetry::OpenTelemetrySpanExt;
use validator::Validate;

use crate::{configuration::app_state::AppState, errors::app_errors::AppError};

use super::dtos::stub_entity_dtos::{StubEntityAddDto, StubEntityUpdateDto};
use tracing::Instrument;

#[axum::debug_handler]
// #[tracing::instrument(
//     skip_all, err
//     // fields(http.uri = %request.uri(), http.method = %request.method(), http.status_code, trace_id, span_id)
// )]
pub async fn list_stub_entity_handler(
    State(state): State<Arc<AppState>>,
) -> Result<(StatusCode, Json<Value>), AppError> {
    let span = create_correlated_span!(Level::INFO, "list_stub_entity_handler");
    async move {
        let use_case = &*state.stub_entity_use_case;
        let stub_entities = use_case.list().await?;
        let json_value = serde_json::to_value(stub_entities)?;
        let body: Json<Value> = Json(json_value);
        log_with_span!(Level::INFO, "list_stub_entity_handler executed");
        Ok((StatusCode::OK, body))
    }
    .instrument(span)
    .await
}

#[axum::debug_handler]
#[tracing::instrument(skip_all)]
pub async fn add_stub_entity_handler(
    State(state): State<Arc<AppState>>,
    WithRejection(Json(payload), _): WithRejection<Json<StubEntityAddDto>, AppError>,
) -> Result<(StatusCode, Json<Value>), AppError> {
    payload.validate()?;
    let use_case = &*state.stub_entity_use_case;
    let mut stub_entity = payload.to_domain();
    let inserted_entity = use_case.add(&mut stub_entity).await?;
    let json_value = serde_json::to_value(inserted_entity)?;
    let body: Json<Value> = Json(json_value);
    log_with_span!(Level::INFO, "add_stub_entity_handler executed");
    Ok((StatusCode::OK, body))
}

#[axum::debug_handler]
#[tracing::instrument(skip_all)]
// #[tracing::instrument(skip_all, err)] Gera log com erro mas muito grande
pub async fn update_stub_entity_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    WithRejection(Json(payload), _): WithRejection<Json<StubEntityUpdateDto>, AppError>,
) -> Result<(StatusCode, Json<Value>), AppError> {
    payload.validate()?;
    let service = &*state.stub_entity_update_service;
    let inserted_entity = service.update(id, payload).await?;
    let json_value = serde_json::to_value(inserted_entity)?;
    let body: Json<Value> = Json(json_value);
    log_with_span!(Level::INFO, "update_stub_entity_handler executed");
    Ok((StatusCode::OK, body))
}

#[axum::debug_handler]
#[tracing::instrument(skip_all)]
pub async fn get_stub_entity_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<Value>), AppError> {
    let use_case = &*state.stub_entity_use_case;
    let retrieved_entity = use_case.get(id, None).await?;
    if retrieved_entity.is_none() {
        let body = json!({
            "message": "Stub entity not found"
        });
        return Ok((StatusCode::NOT_FOUND, Json(body)));
    }
    let json_value = serde_json::to_value(retrieved_entity)?;
    let body: Json<Value> = Json(json_value);
    log_with_span!(Level::INFO, "get_stub_entity_handler executed");
    Ok((StatusCode::OK, body))
}
