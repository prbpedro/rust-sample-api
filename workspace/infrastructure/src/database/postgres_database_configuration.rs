use std::env;

use anyhow::{bail, Result};

pub fn get_db_connection_string() -> Result<String> {
    let key = "DATABASE_CONNECTION_STRING";
    match env::var(key) {
        Ok(val) => Ok(val),
        Err(_) => bail!("Error env var {} not found", key),
    }
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

fn get_u64_env_var(key: &str, default_value: u64) -> Result<u64> {
    get_env_var(key, default_value)
}

fn get_u32_env_var(key: &str, default_value: u32) -> Result<u32> {
    get_env_var(key, default_value)
}

fn get_bool_env_var(key: &str, default_value: bool) -> Result<bool> {
    get_env_var(key, default_value)
}

fn get_env_var<T: std::str::FromStr>(key: &str, default_value: T) -> Result<T> {
    match env::var(key) {
        Ok(val) => match val.parse::<T>() {
            Ok(parsed_val) => Ok(parsed_val),
            Err(_) => bail!("Error parsing env var {}", key),
        },
        Err(_) => Ok(default_value),
    }
}


