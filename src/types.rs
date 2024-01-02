// use std::num::NonZeroU32;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use governor::clock::DefaultClock;
use governor::middleware::NoOpMiddleware;
use governor::state::{direct::NotKeyed, InMemoryState};
use governor::RateLimiter;

pub type TaskMap = Arc<Mutex<HashMap<String, TaskData>>>;
pub type CacheMap = Arc<Mutex<HashMap<String, Vec<Log>>>>;
pub type MyRateLimiter =
    Arc<tokio::sync::Mutex<RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>>>;

use std::{error::Error, fmt};

#[derive(Serialize, Deserialize, Clone)]
pub enum TaskStatus {
    InProgress,
    Completed,
    Failed,
    NotFound,
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

// #[derive(Debug)]
// pub struct CustomError;
// impl Error for CustomError {}
// impl fmt::Display for CustomError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Oh no, something bad went down")
//     }
// }

#[derive(Debug)]
pub enum CustomErrorKind {
    RateLimitExceeded,
    NetworkError,
    DefaultError, // Default error kind
}

#[derive(Debug)]
pub struct CustomError {
    pub kind: CustomErrorKind,
    message: String, // Optional, for additional error information
}

impl CustomError {
    // Convenience constructors for each error kind
    pub fn rate_limit_exceeded() -> Self {
        CustomError {
            kind: CustomErrorKind::RateLimitExceeded,
            message: "Rate limit exceeded".to_string(),
        }
    }

    pub fn network_error() -> Self {
        CustomError {
            kind: CustomErrorKind::NetworkError,
            message: "Network error occurred".to_string(),
        }
    }

    pub fn default_error(msg: &str) -> Self {
        CustomError {
            kind: CustomErrorKind::DefaultError,
            message: msg.to_string(),
        }
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for CustomError {}
