pub mod entities {
    pub mod stub_domain_entity;
}

pub mod ports {
    pub mod repositories {
        pub mod stub_entity_repository_port;
        pub mod transaction_port;
        pub mod database_connection_port;
        pub mod mockserver_http_service_port;
    }
}
