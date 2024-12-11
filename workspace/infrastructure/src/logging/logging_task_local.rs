use tokio::task_local;

#[derive(Clone)]
pub struct RequestData {
    pub correlation_id: String,
    // pub app_name: String,
    // pub app_version: String,
}

task_local! {
    pub static REQUEST_DATA: RequestData;
}

impl RequestData {
    pub fn new(correlation_id: String, 
        // app_name: String, 
        // app_version: String
    ) -> Self {
        Self {
            correlation_id,
            // app_name,
            // app_version,
        }
    }
}
