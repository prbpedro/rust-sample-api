use configuration::{app_runner, tracing_configuration};

pub mod configuration {
    pub mod app_state;
    pub mod routes;
    pub mod tracing_configuration;
    pub mod app_runner;
}

pub mod handlers {
    pub mod stub_entity_handler;
    pub mod dtos {
        pub mod stub_entity_dtos;
    }
}

pub mod errors {
    pub mod app_errors;
}

pub mod use_cases {
    pub mod stub_entity_use_case;
}

pub mod services {
    pub mod stub_entity_update_service;
}

#[tokio::main]
async fn main() {
    //TODO: Middleware for logging and error
    //TODO: Endpoints to delete 
    //TODO: Circuit break + Retry in Database ops
    //TODO: Teste integrado de endpoint
    tracing_configuration::configure_tracing();
    app_runner::run().await;
}
