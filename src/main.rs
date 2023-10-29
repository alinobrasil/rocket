#[macro_use]
extern crate rocket;

use reqwest;

mod routes;
use routes::{check_data, fetch_data, hello_world};
use std::sync::Arc;

mod types;
use types::TaskMap;

#[launch]
fn rocket() -> _ {
    println!("{}", "🚀 The server is ready to accept requests");

    let reqwestClient = Arc::new(reqwest::Client::new());

    rocket::build()
        .manage(TaskMap::default())
        .manage(reqwestClient)
        .mount("/", routes![hello_world, fetch_data, check_data])
}
