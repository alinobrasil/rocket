use rocket::request::{self, FromRequest, Request};
use rocket::{http::Status, serde::json::Json, State};
use std::env;

use serde::Serialize;

use std::error::Error;
use tokio;

use reqwest::{Client, Error as ReqwestError};
// use serde_json::Value;
use std::sync::Arc;

// use crate::chain_data::get_chain_data;

use crate::types::{CacheMap, Log, Response, TaskData, TaskMap, TaskStatus};

use std::collections::HashMap;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

pub struct ApiKey(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        // dotenv().ok();

        // Retrieve the KEYS environment variable and split it into individual keys
        let api_keys = env::var("KEYS").unwrap_or_default();
        let valid_keys: Vec<_> = api_keys.split(',').collect();

        // Check the request for the "x-api-key" header
        match request.headers().get_one("x-api-key") {
            Some(provided_key) => {
                // Check if the provided key is in the list of valid keys
                if valid_keys.contains(&provided_key) {
                    request::Outcome::Success(ApiKey(provided_key.to_string()))
                } else {
                    request::Outcome::Failure((Status::Forbidden, ()))
                }
            }
            None => request::Outcome::Failure((Status::Forbidden, ())),
        }
    }
}

#[get("/hello_world")]
pub fn hello_world(_api_key: ApiKey) -> Result<Json<GenericResponse>, Status> {
    let response = GenericResponse {
        status: "success".to_string(),
        message: "Hello, world!".to_string(),
    };

    Ok(Json(response))
}

#[get("/fetch_data?<block_start>&<block_end>&<contract_address>")]
pub fn fetch_data(
    _api_key: ApiKey,
    block_start: Option<u64>,
    block_end: Option<u64>,
    contract_address: Option<String>,
    map: &State<TaskMap>,
    client: &State<Arc<Client>>,
    cache: &State<CacheMap>,
) -> String {
    // println!("\n***RAW block_start: {:?} \n", block_start);
    // println!("\n***RAW block_end: {:?} \n", block_end);

    //     let _block_start = block_start.unwrap_or(18277200); // Provide default value here
    //     let _block_end = block_end.unwrap_or(18277300); // Provide default value here

    let _block_start = block_start.unwrap_or_else(|| 182770000);
    let _block_end = block_end.unwrap_or_else(|| 182770010);

    let _contract_address = contract_address
        .unwrap_or_else(|| "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".to_string()); // Provide default value here

    // println!("block_start: {}", _block_start);
    // println!("block_end: {}", _block_end);
    // println!("contract_address: {}", _contract_address);

    let task_id = uuid::Uuid::new_v4().to_string();

    let task_id2 = task_id.clone();
    let task_id3 = task_id.clone();

    let client_clone = client.inner().clone();
    let cache_clone = cache.inner().clone();

    let map = map.inner().clone();

    tokio::spawn(async move {
        let data = TaskData {
            status: TaskStatus::InProgress,
            data: None,
        };

        match map.lock() {
            Ok(mut m) => {
                println!("Got lock!");
                m.insert(task_id2, data);
            }
            Err(e) => {
                println!("Failed to get lock {:?}", e);
            }
        };

        // .unwrap().insert(task_id2, data);

        let chain_data: Result<Vec<Log>, Box<dyn Error>> = get_chain_data(
            _block_start,
            _block_end,
            _contract_address,
            client_clone,
            cache_clone,
        )
        .await;
        // println!("task_id2: {}", task_id2);

        match map.lock() {
            Ok(mut m) => {
                println!("Got lock! to write data");
                let task_entry = m.get_mut(&task_id3).unwrap();
                task_entry.status = TaskStatus::Completed;
                // data.data = Some(chain_data);
                match chain_data {
                    Ok(data) => {
                        task_entry.data = Some(data);
                        println!("task_entry data saved");
                    }
                    Err(e) => {
                        task_entry.status = TaskStatus::Error;
                    }
                }
            }
            Err(e) => {
                println!("Failed to get lock {:?}", e);
            }
        };
    });

    task_id
}

#[get("/check_data?<task_id>")]
pub fn check_data(task_id: Option<String>, map: &State<TaskMap>) -> Result<Json<TaskData>, String> {
    let _task_id = task_id.unwrap_or_else(|| "invalid string".to_string());

    let map = map.inner().lock().unwrap();

    if let Some(data) = map.get(&_task_id) {
        Ok(Json(data.clone())) // Successful match, wrapped in Json
    } else {
        Err("Task not found".to_string()) // Error case
    }
}

pub async fn get_chain_data(
    start_block: u64,
    end_block: u64,
    target_address: String,
    client: Arc<Client>,
    cache: CacheMap,
) -> Result<Vec<Log>, Box<dyn Error>> {
    println!("Starting get_chain_data");

    let target_address = target_address.to_lowercase();

    let mut handles = Vec::new();
    let mut current_start = start_block;

    while current_start <= end_block {
        // Define the range for this batch
        let batchsize = 3;
        let current_end = std::cmp::min(current_start + (batchsize - 1), end_block);

        // println!(
        //     "spawning task for blocks {} to {}",
        //     current_start, current_end
        // );

        let client_clone = client.clone();
        let cache_clone = Arc::clone(&cache);

        let handle = tokio::task::spawn(fetch_logs_from_blocks(
            block_to_hex(current_start).to_string(),
            block_to_hex(current_end).to_string(),
            client_clone,
            cache_clone,
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
    // should store these into mutex hashmap
}

async fn fetch_logs_from_blocks(
    start_block: String,
    end_block: String,
    client: Arc<Client>,
    cache: CacheMap,
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

    // todo: check if all target blocks are in cache
    // if so, return cached data
    // otherwise, make the api calls to Infura

    let mut all_blocks_cached = true;
    let mut cached_logs: Vec<Log> = Vec::new();

    //scope to limit duration of lock
    {
        let cache_map = cache.lock().unwrap();

        println!(
            "\nChecking cache for blocks {} to {}",
            start_block, end_block
        );

        let start_block_num = u64::from_str_radix(start_block.trim_start_matches("0x"), 16)
            .expect("Failed to parse start block number");
        let end_block_num = u64::from_str_radix(end_block.trim_start_matches("0x"), 16)
            .expect("Failed to parse end block number");

        for block in start_block_num..=end_block_num {
            let block_hex = block_to_hex(block);
            match cache_map.get(&block_hex) {
                Some(logs) => {
                    println!("Found logs for block {} in cache", block_hex);
                    cached_logs.extend(logs.clone());
                }
                None => {
                    println!("No logs found for block {}", block_hex);
                    all_blocks_cached = false;
                    break;
                }
            }
        }
    }

    if all_blocks_cached {
        return Ok(cached_logs);
    }

    // if not all blocks are cached, go fetch
    println!(
        "\nFetching logs from Infura for blocks {} to {}",
        start_block, end_block
    );
    let infura_url = env::var("INFURA_URL").expect("INFURA_URL must be set");
    let res: Response = client
        .post(&infura_url)
        .json(&payload)
        .send()
        .await?
        .json()
        .await?;

    // put fetched results into cache hashmap
    let logs = res.result.clone();
    let mut grouped_logs: HashMap<String, Vec<Log>> = HashMap::new();

    for log in logs {
        grouped_logs
            .entry(log.blockNumber.clone())
            .or_insert_with(Vec::new)
            .push(log);
    }

    for (block_number, log_group) in grouped_logs {
        cache.lock().unwrap().insert(block_number, log_group);
    }

    Ok(res.result)
}

fn block_to_hex(block: u64) -> String {
    format!("0x{:x}", block)
}

#[get("/check_cache?<block_number>")]
pub fn check_cache(
    block_number: String,
    cache: &State<CacheMap>,
) -> Result<Json<Vec<Log>>, Status> {
    print!("Checking cache...");
    println!("block_number: {}", block_number);

    let block_number_hex = block_to_hex(block_number.parse::<u64>().unwrap());

    let cache_map = cache.inner().lock().unwrap();

    let keys: Vec<_> = cache_map.keys().cloned().collect();
    let keys_str = keys.join(", ");
    println!("Cached blocks: {}", keys_str);

    match cache_map.get(&block_number_hex) {
        Some(logs) => Ok(Json(logs.clone())),
        None => {
            println!("No logs found in cache");
            Err(Status::NotFound)
        }
    }
}
