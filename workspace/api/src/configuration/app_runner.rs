use anyhow::{bail, Result};
use infrastructure::database::migrations::migrator::Migrator;
use tracing::{error, info};

use crate::{configuration::routes, errors::app_errors::UnexpectedError};

use super::app_state::AppState;

pub async fn run() -> Result<()> {
    let state = create_app_state().await?;

    execute_migrations(&state).await?;

    let app = routes::build_routes(state).await;
    let port = "3000";
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    info!(
        app.name = %env!("CARGO_PKG_NAME"),
        app.version = %env!("CARGO_PKG_VERSION"),
        "Application listening on port {}", 
        port);

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn create_app_state() -> Result<std::sync::Arc<AppState>> {
    let state_result = AppState::new().await;
    let state = match state_result {
        Ok(state) => {
            log_info("App state created successfully");
            state
        },
        Err(err) => {
            log_error("Failed to create application state: {}", err);
            bail!("Failed to create application state")
        }
    };
    Ok(state)
}

async fn execute_migrations(state: &std::sync::Arc<AppState>) -> Result<()>{
    match Migrator::run_migrations(&state.database_connection.conn).await {
        Ok(_) => {
            log_info("Database migrations ran successfully");
            Ok(())
        },
        Err(err) => {
            log_error("Failed to run database migrations: {}", err);
            bail!("Failed to run database migrations")
        },
    }
}

fn log_info(message: &str) {
    info!(
        app.name = %env!("CARGO_PKG_NAME"),
        app.version = %env!("CARGO_PKG_VERSION"),
        message);
}

fn log_error(msg: &str, err: anyhow::Error) {
    let error_message = err.to_string();
    let app_error: UnexpectedError = UnexpectedError::new(err);

    error!(
        backtrace = app_error.get_truncated_backtrace(),
        app.name = %env!("CARGO_PKG_NAME"),
        app.version = %env!("CARGO_PKG_VERSION"),
        msg, 
        error_message
    );
}
