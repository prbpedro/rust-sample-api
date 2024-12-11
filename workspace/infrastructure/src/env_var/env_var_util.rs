use std::env;

use anyhow::{bail, Result};

pub fn get_required_string_env_var(key: &str) -> std::result::Result<String, anyhow::Error> {
    match env::var(key) {
        Ok(val) => Ok(val),
        Err(_) => bail!("Error env var {} not found", key),
    }
}

pub fn get_u64_env_var(key: &str, default_value: u64) -> Result<u64> {
    get_env_var(key, default_value)
}

pub fn get_u32_env_var(key: &str, default_value: u32) -> Result<u32> {
    get_env_var(key, default_value)
}

pub fn get_bool_env_var(key: &str, default_value: bool) -> Result<bool> {
    get_env_var(key, default_value)
}

pub fn get_vec_env_var<T: std::str::FromStr>(key: &str, default_value: Vec<T>) -> Result<Vec<T>> {
    match env::var(key) {
        Ok(val) => {
            let mut vec = Vec::new();
            for item in val.split(',') {
                match item.parse::<T>() {
                    Ok(parsed_item) => vec.push(parsed_item),
                    Err(_) => bail!("Error parsing env var {} item", key),
                }
            }
            Ok(vec)
        }
        Err(_) => Ok(default_value),
    }
}

pub fn get_env_var<T: std::str::FromStr>(key: &str, default_value: T) -> Result<T> {
    match env::var(key) {
        Ok(val) => match val.parse::<T>() {
            Ok(parsed_val) => Ok(parsed_val),
            Err(_) => bail!("Error parsing env var {}", key),
        },
        Err(_) => Ok(default_value),
    }
}

