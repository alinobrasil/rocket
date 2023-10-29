use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub type TaskMap = Arc<Mutex<HashMap<String, String>>>;

pub enum TaskStatus {
    InProgress,
    Completed,
    Error,
}

pub struct TaskData {
    status: TaskStatus,
    data: Option<String>,
}
