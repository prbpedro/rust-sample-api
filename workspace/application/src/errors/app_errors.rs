use core::fmt;

use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use infrastructure::log_with_span;
use infrastructure::logging::logging_util::REQUEST_DATA;
use opentelemetry::trace::TraceContextExt;
use sea_orm::DbErr;
use serde_json::{json, Value};
use tracing::Level;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use validator::ValidationErrors;

#[derive(Debug)]
pub enum AppError {
    UnexpectedError(UnexpectedError),
    ValidationError(ValidationErrors),
    JsonRejection(JsonRejection),
    UnprocessableEntity(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AppError: {:?}", self)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::UnexpectedError(e) => {
                log_with_span!(
                    Level::ERROR,
                    backtrace = %e.get_truncated_backtrace(),
                    "An error occurred: {}", e.cause
                );

                tracing::Span::current()
                    .record("status_code", StatusCode::INTERNAL_SERVER_ERROR.as_u16());

                build_error_response(
                    String::from("Unexpected error"),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
                .into_response()
            }
            AppError::ValidationError(errors) => {
                let message = extract_validation_errors(&errors);
                let body = json!({
                    "errors": message
                });
                (StatusCode::BAD_REQUEST, axum::Json(body)).into_response()
            }
            AppError::JsonRejection(e) => {
                build_error_response(e.body_text(), e.status()).into_response()
            }
            AppError::UnprocessableEntity(message) => {
                build_error_response(message, StatusCode::UNPROCESSABLE_ENTITY).into_response()
            }
        }
    }
}

fn build_error_response(message: String, status_code: StatusCode) -> (StatusCode, Json<Value>) {
    let body = json!({
        "message": message
    });
    (status_code, Json(body))
}

fn extract_validation_errors(
    errors: &ValidationErrors,
) -> serde_json::Map<String, serde_json::Value> {
    let mut json_errors: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();

    for error in errors.errors() {
        match error.1 {
            validator::ValidationErrorsKind::Field(errors) => {
                let mut field_errors: Vec<String> = Vec::new();

                for field_error in errors {
                    field_errors.push(field_error.to_string());
                }
                json_errors.insert(
                    error.0.to_string(),
                    serde_json::Value::Array(
                        field_errors
                            .into_iter()
                            .map(serde_json::Value::String)
                            .collect(),
                    ),
                );
            }
            validator::ValidationErrorsKind::Struct(errors) => {
                let field_errors = extract_validation_errors(&errors);

                json_errors.insert(error.0.to_string(), field_errors.into());
            }
            validator::ValidationErrorsKind::List(errors) => {
                let mut field_errors: Vec<String> = Vec::new();
                errors.iter().for_each(|(index, errors)| {
                    field_errors.push(format!("{}: {:?}", index, errors));
                });
                json_errors.insert(
                    error.0.to_string(),
                    serde_json::Value::Array(
                        field_errors
                            .into_iter()
                            .map(serde_json::Value::String)
                            .collect(),
                    ),
                );
            }
        };
    }

    json_errors
}

#[derive(Debug)]
pub struct UnexpectedError {
    pub cause: anyhow::Error,
}

impl UnexpectedError {
    pub fn new(cause: anyhow::Error) -> Self {
        UnexpectedError { cause }
    }

    pub fn get_truncated_backtrace(&self) -> String {
        let stack_trace = self.cause.backtrace().to_string();
        if stack_trace.len() > 2000 {
            stack_trace[..2000].to_string()
        } else {
            stack_trace
        }
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        if let Some(db_error) = err.downcast_ref::<DbErr>() {
            if db_error.to_string().contains("fk-stub-table-ref") {
                return AppError::UnprocessableEntity("auto_ref does not exist".to_string());
            }
        }

        AppError::UnexpectedError(UnexpectedError { cause: err })
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::UnexpectedError(UnexpectedError {
            cause: anyhow::Error::new(err),
        })
    }
}

impl From<JsonRejection> for AppError {
    fn from(rejection: JsonRejection) -> Self {
        AppError::JsonRejection(rejection)
    }
}

impl From<ValidationErrors> for AppError {
    fn from(errors: ValidationErrors) -> Self {
        AppError::ValidationError(errors)
    }
}
