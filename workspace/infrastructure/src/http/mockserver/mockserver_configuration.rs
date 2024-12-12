use anyhow::Result;

use crate::env_var::env_var_util::get_required_string_env_var;

pub fn get_mockserver_base_url() -> Result<String> {
    get_required_string_env_var("MOCKSERVER_BASE_URL")
}