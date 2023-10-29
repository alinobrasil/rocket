use rocket::{http::Status, serde::json::Json, State};

use serde::Serialize;

use crate::types::TaskMap;

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

    // let map = map.inner().clone();
    // tokio::spawn(async move {
    //     let data = fetch_data_async().await;
    //     map.lock().unwrap().insert(task_id, data);
    // });

    task_id
}
