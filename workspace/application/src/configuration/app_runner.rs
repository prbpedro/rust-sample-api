use infrastructure::database::migrations::migrator::Migrator;
use tracing::{error, info};

use crate::{configuration::routes, errors::app_errors::UnexpectedError};

use super::app_state::AppState;

pub async fn run() {

    let state_result = AppState::new().await;
    let state = match state_result {
        Ok(state) => state,
        Err(err) => {
            log_error("Failed to create application state: {}", err);
            return;
        }
    };
    
    if let Err(err) = Migrator::run_migrations(&state.database_connection.conn).await {
        log_error( "Failed to run database migrations: {}", err);
        return;
    }
    
    let app = routes::build_routes(state).await;
    let port = "3000";
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    info!("Application listening on port {}", port);
    axum::serve(listener, app).await.unwrap();
}

fn log_error(msg: &str, err: anyhow::Error) {
    let error_message = err.to_string();
    let app_error: UnexpectedError = UnexpectedError::new(err);
    error!(
        backtrace = app_error.get_truncated_backtrace(),
        msg, error_message
    );
}
