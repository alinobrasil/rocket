use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub type TaskMap = Arc<Mutex<HashMap<String, TaskData>>>;

#[derive(Serialize, Deserialize, Clone)]
pub enum TaskStatus {
    InProgress,
    Completed,
    Error,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TaskData {
    pub status: TaskStatus,
    pub data: Option<Vec<Log>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Log {
    pub address: String,
    pub data: String,
    pub topics: Vec<String>,
    pub blockNumber: String, //added block number for easier hashmap management
}

#[derive(Debug, Deserialize)]
pub struct Response {
    pub jsonrpc: String,
    pub id: u32,
    pub result: Vec<Log>,
}
