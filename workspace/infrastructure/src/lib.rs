pub mod database {
    pub mod migrations {
        mod m20241126_000001_create_stub_table;
        pub mod migrator;
    }

    pub mod repositories {
        pub mod stub_entity_sea_orm_postgres_repository;
        pub mod database_data_seaorm;
        pub mod database_data;
    }

    pub mod entities {
        pub mod stub_database_entity;
    }   

    mod postgres_database_configuration;

}

pub mod logging {
    pub mod logging_util;
}