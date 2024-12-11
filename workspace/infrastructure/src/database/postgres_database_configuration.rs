use anyhow::Result;

use crate::env_var::env_var_util::{
    get_bool_env_var, get_required_string_env_var, get_u32_env_var, get_u64_env_var,
};

pub fn get_db_connection_string() -> Result<String> {
    get_required_string_env_var("DATABASE_CONNECTION_STRING")
}

pub fn get_db_max_connections() -> Result<u32> {
    get_u32_env_var("DATABASE_MAX_CONNECTIONS", 10)
}

pub fn get_db_min_connections() -> Result<u32> {
    get_u32_env_var("DATABASE_MIN_CONNECTIONS", 2)
}

pub fn get_db_connect_timeout_seconds() -> Result<u64> {
    get_u64_env_var("DATABASE_CONNECT_TIMEOUT_SECONDS", 2)
}

pub fn get_db_idle_connection_timeout_seconds() -> Result<u64> {
    get_u64_env_var("DATABASE_IDLE_CONNECTION_TIMEOUT_SECONDS", 600)
}

pub fn get_db_max_lifetime_connection_seconds() -> Result<u64> {
    get_u64_env_var("DATABASE_MAX_LIFETIME_CONNECTION_SECONDS", 1800)
}

pub fn get_db_sqlx_logging() -> Result<bool> {
    get_bool_env_var("DATABASE_SQLX_LOGGING", false)
}
