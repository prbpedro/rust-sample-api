use infrastructure::env_var::env_var_util::get_vec_env_var;
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};

pub fn setup_metrics_recorder() -> PrometheusHandle {
    let exponential_seconds = get_vec_env_var(
        "HTTP_REQUEST_METRICS_EXPONENTIAL_SECONDS",
        vec![0.5, 0.75, 1.0],
    )
    .unwrap();

    PrometheusBuilder::new()
        .set_buckets_for_metric(
            Matcher::Full("http_requests_duration_seconds".to_string()),
            &exponential_seconds,
        )
        .unwrap()
        .install_recorder()
        .unwrap()
}
