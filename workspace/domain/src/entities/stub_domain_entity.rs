use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct StubEntity {
    pub id: Option<i32>,
    pub name: String,
    pub value: KeyValue,
    pub auto_ref: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyValue {
    pub id: i32,
    pub name: String,
}