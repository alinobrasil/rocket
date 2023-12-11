#[macro_use]
extern crate rocket;
// use rocket::data::Outcome;
use reqwest;

mod routes;
use routes::{check_cache, check_data, fetch_data, hello_world};
use std::sync::Arc;

mod types;
use dotenv::dotenv;
use types::{CacheMap, TaskMap};

#[launch]
fn rocket() -> _ {
    println!("{}", "🚀 The server is ready to accept requests");
    dotenv().expect("Cannot load env");

    let reqwestClient = Arc::new(reqwest::Client::new());

    rocket::build()
        .manage(TaskMap::default())
        .manage(reqwestClient)
        .manage(CacheMap::default())
        .mount(
            "/",
            routes![hello_world, fetch_data, check_data, check_cache],
        )
}
