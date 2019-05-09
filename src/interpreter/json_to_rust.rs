use serde::{Deserialize, Serialize};
// use serde_json::Value;
use multimap::MultiMap;

// Node module
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsContext {
    pub past: Option< Vec<serde_json::Value> >,
    pub current: Option< Vec<serde_json::Value> >,
    pub metadata: Option< Vec<serde_json::Value> >,
}

// -----------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemoryType {
    pub created_at: String,
    pub flow_name: Option<String>,
    pub key: String,
    pub step_name: Option<String>,
    pub r#type: Option<String>,
    pub value: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Memory {
    pub past: MultiMap<String, MemoryType>,
    pub current: MultiMap<String, MemoryType>,
    pub metadata: MultiMap<String, MemoryType>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PayLoadContent {
    pub text: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PayLoad {
    pub content_type: String,
    pub content: PayLoadContent
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub channel_id: String,
    pub channel_type: String,
    pub user_id: String,
    pub timestamp: i64,
    pub payload: PayLoad
}