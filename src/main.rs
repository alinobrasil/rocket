#[macro_use]
extern crate rocket;
// use rocket::data::Outcome;
// use reqwest;

mod routes;
use routes::{check_cache, check_data, fetch_data, hello_world};
use std::sync::Arc;

mod types;
use dotenv::dotenv;
use tokio::sync::Semaphore;
use types::{CacheMap, TaskMap};

use governor::{Quota, RateLimiter};
use nonzero_ext::nonzero;

use tokio::sync::Mutex;

#[launch]
fn rocket() -> _ {
    println!("🚀 The server is ready to accept requests");
    dotenv().expect("Cannot load env");

    let reqwest_client = Arc::new(reqwest::Client::new());
    let semaphore = Arc::new(Semaphore::new(10000)); //limit concurrent requests

    let rate_limiter = Arc::new(Mutex::new(RateLimiter::direct(Quota::per_second(
        nonzero!(10u32),
    ))));

    rocket::build()
        .manage(TaskMap::default())
        .manage(reqwest_client)
        .manage(CacheMap::default())
        .manage(semaphore)
        .manage(rate_limiter)
        .mount(
            "/",
            routes![hello_world, fetch_data, check_data, check_cache],
        )
}
