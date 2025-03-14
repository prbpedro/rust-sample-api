use anyhow::Result;
use aws_config::{retry::RetryConfig, BehaviorVersion};
use domain::ports::{
    messaging::messaging_service_port::MessagingServicePort,
    repositories::{
        mockserver_http_service_port::MockserverHttpServicePort,
        stub_entity_repository_port::StubEntityRepositoryPort,
    },
};
use infrastructure::{
    database::repositories::{
        database_data::DatabaseConnection,
        stub_entity_sea_orm_postgres_repository::StubEntitySeaOrmPostgresRepository,
    },
    http::mockserver::{
        mockserver_configuration::get_mockserver_base_url,
        mockserver_http_service::MockserverHttpService,
    },
    messaging::{
        aws_sqs_messaging_configuration::get_rust_test_aws_sqs_queue_url,
        aws_sqs_messaging_service::AwsSqsMessagingService,
    },
};
use std::{sync::Arc, time::Duration};

use crate::{
    services::stub_entity_update_service::StubEntityUpdateService,
    use_cases::stub_entity_use_case::StubEntityUseCase,
};

pub struct AppState {
    pub database_connection: Arc<DatabaseConnection<sea_orm::DatabaseConnection>>,
    pub stub_entity_use_case: Arc<StubEntityUseCase>,
    pub stub_entity_update_service: Arc<StubEntityUpdateService>,
    pub aws_client: Arc<aws_sdk_sqs::Client>,
    pub messaging_service: Arc<dyn MessagingServicePort>,
}

impl AppState {
    pub async fn new() -> Result<Arc<AppState>> {
        let database_connection = DatabaseConnection::new().await?;

        let stub_entity_repository = build_stub_entity_repository(&database_connection);

        let mockserver_http_service = build_mock_server_http_service();

        let retry_config = RetryConfig::standard()
            .with_max_attempts(10)
            .with_initial_backoff(Duration::from_millis(1))
            .with_max_backoff(Duration::from_secs(20));
        let retry_config = retry_config;

        let config = aws_config::defaults(BehaviorVersion::latest())
            .retry_config(retry_config)
            .load()
            .await;

        let aws_client: Arc<aws_sdk_sqs::Client> = Arc::new(aws_sdk_sqs::Client::new(&config));

        let messaging_service = build_messaging_service(&aws_client).await;

        let stub_entity_use_case = build_stub_entity_use_case(
            &stub_entity_repository,
            &mockserver_http_service,
            &messaging_service,
        );

        let stub_entity_update_service =
            build_stub_entity_update_service(&stub_entity_use_case, &database_connection);

        let app_state = Self {
            database_connection,
            stub_entity_use_case,
            stub_entity_update_service,
            aws_client,
            messaging_service
        };

        Ok(Arc::new(app_state))
    }
}

fn build_mock_server_http_service() -> Arc<dyn MockserverHttpServicePort> {
    let reqwest_client = Arc::new(reqwest::Client::new());
    let base_url = get_mockserver_base_url().unwrap();
    Arc::new(MockserverHttpService::new(reqwest_client, base_url))
}

fn build_stub_entity_repository(
    database_connection: &Arc<DatabaseConnection<sea_orm::DatabaseConnection>>,
) -> Arc<dyn StubEntityRepositoryPort> {
    Arc::new(StubEntitySeaOrmPostgresRepository::new(
        database_connection.clone(),
    ))
}

fn build_stub_entity_update_service(
    stub_entity_use_case: &Arc<StubEntityUseCase>,
    database_connection: &Arc<DatabaseConnection<sea_orm::DatabaseConnection>>,
) -> Arc<StubEntityUpdateService> {
    Arc::new(StubEntityUpdateService::new(
        stub_entity_use_case.clone(),
        database_connection.clone(),
    ))
}

fn build_stub_entity_use_case(
    repository: &Arc<dyn StubEntityRepositoryPort>,
    mockserver_http_service: &Arc<dyn MockserverHttpServicePort>,
    messaging_service: &Arc<dyn MessagingServicePort>,
) -> Arc<StubEntityUseCase> {
    Arc::new(StubEntityUseCase::new(
        repository.clone(),
        mockserver_http_service.clone(),
        messaging_service.clone(),
    ))
}

async fn build_messaging_service(
    aws_client: &Arc<aws_sdk_sqs::Client>,
) -> Arc<dyn MessagingServicePort> {
    let aws_sqs_queue_url = get_rust_test_aws_sqs_queue_url().unwrap();
    Arc::new(AwsSqsMessagingService::new(
        aws_client.clone(),
        aws_sqs_queue_url.clone(),
    ))
}
