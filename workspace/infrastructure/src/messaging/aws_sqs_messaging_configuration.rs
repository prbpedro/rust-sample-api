use anyhow::Result;

use crate::env_var::env_var_util::get_required_string_env_var;

pub fn get_rust_test_aws_sqs_queue_url() -> Result<String> {
    get_required_string_env_var("RUST_TEST_AWS_SQS_QUEUE_URL")
}