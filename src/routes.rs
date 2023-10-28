use rocket::{http::Status, serde::json::Json};

use serde::Serialize;

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
