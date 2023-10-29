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
    pub data: Option<String>,
}
