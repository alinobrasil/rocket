use rocket::{http::Status, serde::json::Json, State};

use serde::Serialize;
use std::env;
use std::error::Error;
use tokio;

use dotenv::dotenv;
use reqwest::{Client, Error as ReqwestError};
use serde_json::Value;
use std::sync::Arc;

// use crate::chain_data::get_chain_data;

use crate::types::{Log, Response, TaskData, TaskMap, TaskStatus};

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
pub fn fetch_data(map: &State<TaskMap>, client: &State<Arc<Client>>) -> String {
    let task_id = uuid::Uuid::new_v4().to_string();

    let task_id2 = task_id.clone();
    let client_clone = client.inner().clone();

    let map = map.inner().clone();
    tokio::spawn(async move {
        let data = TaskData {
            status: TaskStatus::InProgress,
            data: None,
        };
        map.lock().unwrap().insert(task_id2, data);

        let chain_data = get_chain_data(
            18277200,
            18277208,
            "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".to_string(),
            &client_clone,
        );
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

pub async fn get_chain_data(
    start_block: u64,
    end_block: u64,
    target_address: String,
    client: <Arc<Client>>,
) -> Result<Vec<Log>, Box<dyn Error>> {
    let target_address = target_address.to_lowercase();

    let mut handles = Vec::new();
    let mut current_start = start_block;

    while current_start <= end_block {
        // Define the range for this batch
        let batchsize = 3;
        let current_end = std::cmp::min(current_start + (batchsize - 1), end_block);

        println!(
            "spawning task for blocks {} to {}",
            current_start, current_end
        );

        let client_clone = Arc::clone(client.inner());

        let handle = tokio::task::spawn(fetch_logs_from_blocks(
            block_to_hex(current_start).to_string(),
            block_to_hex(current_end).to_string(),
            client_clone,
        ));
        handles.push(handle);

        current_start += batchsize;
    }

    let mut matching_logs = Vec::new();

    // Await on the handles to get the results
    for handle in handles {
        match handle.await {
            Ok(Ok(logs)) => {
                // Filter logs based on the specified address and collect them
                let filtered_logs: Vec<_> = logs
                    .into_iter()
                    .filter(|log| log.address == target_address)
                    .collect();

                matching_logs.extend(filtered_logs.clone());
                println!("# matching entries: {}", filtered_logs.len());
            }
            Ok(Err(e)) => {
                println!("Error fetching logs: {}", e);
                return Err(Box::new(e));
            }
            Err(e) => {
                println!("Task error: {}", e);
                return Err(Box::new(e));
            }
        }
    }

    Ok(matching_logs)
}

async fn fetch_logs_from_blocks(
    start_block: String,
    end_block: String,
    client: Arc<Client>,
) -> Result<Vec<Log>, ReqwestError> {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "eth_getLogs",
        "params": [{
            "fromBlock": start_block,
            "toBlock": end_block
        }]
    });

    dotenv().ok();

    // let client = reqwest::Client::new();
    let infura_url = env::var("INFURA_URL").expect("INFURA_URL must be set");
    let res: Response = client
        .post(&infura_url)
        .json(&payload)
        .send()
        .await?
        .json()
        .await?;

    Ok(res.result)
}

fn block_to_hex(block: u64) -> String {
    format!("0x{:x}", block)
}
