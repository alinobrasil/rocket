use rocket::{http::Status, serde::json::Json, State};
use serde::Serialize;
use tokio;

use crate::types::{TaskData, TaskMap, TaskStatus};

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[get("/hello_world")]
pub fn hello_world() -> Result<Json<GenericResponse>, Status> {
    let response = GenericResponse {
        status: "success".to_string(),
        message: "Hello, world!".to_string(),
    };

    Ok(Json(response))
}

#[get("/fetch_data")]
pub fn fetch_data(map: &State<TaskMap>) -> String {
    let task_id = uuid::Uuid::new_v4().to_string();

    let task_id2 = task_id.clone();

    let map = map.inner().clone();
    tokio::spawn(async move {
        let data = TaskData {
            status: TaskStatus::InProgress,
            data: None,
        };
        map.lock().unwrap().insert(task_id2, data);
    });

    task_id
}

#[get("/check_data/<task_id>")]
pub fn check_data(task_id: &str, map: &State<TaskMap>) -> Result<Json<TaskData>, String> {
    let map = map.inner().lock().unwrap();

    if let Some(data) = map.get(task_id) {
        Ok(Json(data.clone())) // Successful match, wrapped in Json
    } else {
        Err("Task not found".to_string()) // Error case
    }
}
